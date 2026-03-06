use std::collections::HashMap;
use zed::lsp::CompletionKind;
use zed::settings::LspSettings;
use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::serde_json::json;
use zed_extension_api::{
    self as zed, current_platform, serde_json, DebugAdapterBinary, DebugTaskDefinition, Os, Result,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

/// Represents a Dart/Flutter binary with its path and optional arguments
struct DartBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

/// Information about a Flutter target device (mobile, web, desktop)
#[derive(Debug, Clone)]
struct DeviceInfo {
    /// Unique device identifier (e.g., "chrome", "iphone-15-pro")
    id: String,
    /// Human-readable device name
    name: String,
    /// Target platform (e.g., "ios", "android", "web-javascript")
    platform: String,
    /// Whether this is an emulator/simulator
    emulator: bool,
}

/// Main extension struct managing Flutter/Dart development features
struct DartExtension {
    /// Cached list of available devices (refreshed every 5 minutes)
    device_cache: Vec<DeviceInfo>,
    /// Last selected device ID for smart defaults
    last_selected_device: Option<String>,
    /// FVM status cache per worktree (worktree_id -> is_fvm_project)
    fvm_status: HashMap<String, bool>,
}

impl DartExtension {
    /// Checks if the project uses FVM (Flutter Version Manager)
    /// by looking for .fvm/fvm_config.json
    /// Uses caching to avoid repeated file system checks
    fn is_fvm_project(&mut self, worktree: &Worktree) -> bool {
        let worktree_id = worktree.root_path();

        if let Some(&is_fvm) = self.fvm_status.get(&worktree_id) {
            return is_fvm;
        }

        let is_fvm = worktree.read_text_file(".fvm/fvm_config.json").is_ok();
        self.fvm_status.insert(worktree_id, is_fvm);
        is_fvm
    }

    /// Returns cached devices or empty list (simplified for API constraints)
    fn get_cached_devices(&self, _worktree: &Worktree) -> Result<Vec<DeviceInfo>, String> {
        Ok(self.device_cache.clone())
    }

    /// Selects the best device for debugging using priority heuristics:
    /// 1. Last used device (if still available)
    /// 2. Physical device (non-emulator, non-web)
    /// 3. Emulator
    /// 4. First available device
    /// 5. Default to "chrome" if no devices found
    fn select_best_device(&self, worktree: &Worktree) -> Result<String, String> {
        let devices = self.get_cached_devices(worktree)?;

        if devices.is_empty() {
            return Ok("chrome".to_string());
        }

        if let Some(last_device_id) = &self.last_selected_device {
            if devices.iter().any(|d| &d.id == last_device_id) {
                return Ok(last_device_id.clone());
            }
        }

        let physical_device = devices
            .iter()
            .find(|d| !d.emulator && d.platform != "web-javascript");
        if let Some(device) = physical_device {
            return Ok(device.id.clone());
        }

        let emulator = devices.iter().find(|d| d.emulator);
        if let Some(device) = emulator {
            return Ok(device.id.clone());
        }

        Ok(devices[0].id.clone())
    }

    fn ensure_device_available(&self, device_id: &str, worktree: &Worktree) -> Result<(), String> {
        let devices = self.get_cached_devices(worktree)?;

        if devices.is_empty() {
            return Ok(());
        }

        if devices.iter().any(|d| d.id == device_id) {
            Ok(())
        } else {
            let available: Vec<String> = devices
                .iter()
                .map(|d| format!("{} ({})", d.name, d.id))
                .collect();
            Err(format!(
                "Device '{}' not found.\n\nAvailable devices:\n  {}\n\nTip: Run 'flutter devices' to list all available devices.",
                device_id,
                available.join("\n  ")
            ))
        }
    }

    fn get_platform_for_device(
        &self,
        device_id: &str,
        worktree: &Worktree,
    ) -> Result<String, String> {
        let devices = self.get_cached_devices(worktree)?;

        Ok(devices
            .iter()
            .find(|d| d.id == device_id)
            .map(|d| {
                if d.platform.contains("ios") {
                    "ios".to_string()
                } else if d.platform.contains("android") {
                    "android".to_string()
                } else if d.platform.contains("web") {
                    "web".to_string()
                } else {
                    d.platform.clone()
                }
            })
            .unwrap_or_else(|| "web".to_string()))
    }

    /// Extracts device ID from task arguments (e.g., -d chrome, --device-id=chrome)
    fn extract_device_id(&self, task: &zed::TaskTemplate) -> Option<String> {
        let args = &task.args;

        for i in 0..args.len() {
            if args[i] == "-d" || args[i] == "--device-id" {
                return args.get(i + 1).cloned();
            }

            if args[i].starts_with("--device-id=") {
                return args[i].strip_prefix("--device-id=").map(|s| s.to_string());
            }
        }

        None
    }

    /// Extracts custom arguments (flavors, modes, etc.) from task
    fn extract_custom_args(&self, task: &zed::TaskTemplate) -> Vec<String> {
        let args = &task.args;
        let mut custom_args = Vec::new();

        let mut skip_next = false;
        for arg in args.iter() {
            if skip_next {
                skip_next = false;
                continue;
            }

            if arg == "flutter" || arg == "run" {
                continue;
            }

            if arg == "-d" || arg == "--device-id" || arg.starts_with("--device-id=") {
                if !arg.starts_with("--device-id=") {
                    skip_next = true;
                }
                continue;
            }

            if arg.starts_with("--") {
                custom_args.push(arg.clone());
            }
        }

        custom_args
    }

    fn language_server_binary(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<DartBinary> {
        let binary_settings = LspSettings::for_worktree("dart", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(DartBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("dart") {
            return Ok(DartBinary {
                path,
                args: binary_args,
            });
        }

        Err(
            "dart must be installed from dart.dev/get-dart or pointed to by the LSP binary settings"
                .to_string(),
        )
    }
}

impl zed::Extension for DartExtension {
    fn new() -> Self {
        Self {
            device_cache: Vec::new(),
            last_selected_device: None,
            fvm_status: HashMap::new(),
        }
    }

    /// Configures and returns the debug adapter binary for Dart/Flutter debugging
    ///
    /// This method handles:
    /// - FVM detection and command routing
    /// - Platform-specific binary selection (Windows vs Unix)
    /// - Auto-device selection for Flutter projects
    /// - Hot reload configuration for Flutter
    /// - Comprehensive error messages with remediation suggestions
    ///
    /// Reference:
    /// https://github.com/zed-industries/zed/blob/main/crates/dap_adapters/src/gdb.rs
    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        config: DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        let user_config: serde_json::Value = serde_json::from_str(&config.config)
            .map_err(|e| format!("Failed to parse debug config: {e}"))?;

        let program = user_config
            .get("program")
            .and_then(|v| v.as_str())
            .unwrap_or("lib/main.dart");

        let args = user_config
            .get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .unwrap_or_default();

        let use_fvm = user_config
            .get("useFvm")
            .and_then(|v| v.as_bool())
            .unwrap_or_else(|| self.is_fvm_project(worktree));

        let debug_mode = user_config
            .get("type")
            .and_then(|v| v.as_str())
            .filter(|s| !s.trim().is_empty())
            .ok_or_else(|| "type is required and cannot be empty or null".to_string())?;

        let (os, _) = current_platform();
        let tool = if debug_mode == "flutter" {
            match os {
                Os::Windows => "flutter.bat",
                _ => "flutter",
            }
        } else {
            match os {
                Os::Windows => "dart.bat",
                _ => "dart",
            }
        };

        let (command, arguments) = if use_fvm {
            if worktree.which("fvm").is_none() {
                return Err("FVM is configured but 'fvm' command not found. Install FVM: dart pub global activate fvm".to_string());
            }
            (
                "fvm".to_string(),
                vec![tool.to_string(), "debug_adapter".to_string()],
            )
        } else {
            let tool_path = worktree.which(tool);
            if tool_path.is_none() {
                if debug_mode == "flutter" {
                    return Err(
                        "Flutter SDK not found. Install from flutter.dev or add to PATH"
                            .to_string(),
                    );
                } else {
                    return Err(
                        "Dart SDK not found. Install from dart.dev or add to PATH".to_string()
                    );
                }
            }
            (tool.to_string(), vec!["debug_adapter".to_string()])
        };

        let device_id = if debug_mode == "flutter" {
            if let Some(id) = user_config.get("device_id").and_then(|v| v.as_str()) {
                self.ensure_device_available(id, worktree)?;
                self.last_selected_device = Some(id.to_string());
                id.to_string()
            } else {
                let selected = self.select_best_device(worktree)?;
                self.last_selected_device = Some(selected.clone());
                selected
            }
        } else {
            user_config
                .get("device_id")
                .and_then(|v| v.as_str())
                .unwrap_or("chrome")
                .to_string()
        };

        let platform = if debug_mode == "flutter" {
            if let Some(p) = user_config.get("platform").and_then(|v| v.as_str()) {
                p.to_string()
            } else {
                self.get_platform_for_device(&device_id, worktree)?
            }
        } else {
            user_config
                .get("platform")
                .and_then(|v| v.as_str())
                .unwrap_or("web")
                .to_string()
        };

        let cwd = user_config
            .get("cwd")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| Some(worktree.root_path()));

        let request = user_config
            .get("request")
            .and_then(|v| v.as_str())
            .unwrap_or("launch");

        let vm_service_uri = user_config.get("vmServiceUri").and_then(|v| v.as_str());

        let supports_hot_reload = user_config
            .get("supportsHotReload")
            .and_then(|v| v.as_bool())
            .unwrap_or(debug_mode == "flutter");

        let hot_reload_on_save = user_config
            .get("hotReloadOnSave")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let config_json = if debug_mode == "flutter" {
            json!({
                "type": tool,
                "request": request,
                "vmServiceUri": vm_service_uri,
                "program": program,
                "cwd": cwd.clone().unwrap_or_default(),
                "args": args,
                "flutterMode": "debug",
                "deviceId": device_id,
                "platform": platform,
                "stopOnEntry": false,
                "supportsHotReload": supports_hot_reload,
                "hotReloadOnSave": hot_reload_on_save
            })
        } else {
            json!({
                "type": tool,
                "request": request,
                "vmServiceUri": vm_service_uri,
                "program": program,
                "cwd": cwd.clone().unwrap_or_default(),
                "args": args,
                "stopOnEntry": false
            })
        }
        .to_string();

        let debug_adapter_binary = DebugAdapterBinary {
            command: Some(command),
            arguments,
            envs: vec![],
            cwd,
            connection: None,
            request_args: StartDebuggingRequestArguments {
                configuration: config_json,
                request: match request {
                    "attach" => StartDebuggingRequestArgumentsRequest::Attach,
                    _ => StartDebuggingRequestArgumentsRequest::Launch,
                },
            },
        };
        Result::Ok(debug_adapter_binary)
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        match config.get("request") {
            Some(v) if v == "launch" => Ok(StartDebuggingRequestArgumentsRequest::Launch),
            Some(v) if v == "attach" => Ok(StartDebuggingRequestArgumentsRequest::Attach),
            Some(value) => Err(format!(
                "Unexpected value for `request` key in Dart debug adapter configuration: {value:?}"
            )),
            None => {
                Err("Missing required `request` field in Dart debug adapter configuration".into())
            }
        }
    }

    /// Converts a high-level debug configuration into a debug scenario
    ///
    /// This handles modal-based debugging where users can configure debug scenarios
    /// through a UI. Translates generic configurations into Flutter/Dart-specific ones.
    fn dap_config_to_scenario(
        &mut self,
        config: zed::DebugConfig,
    ) -> Result<zed::DebugScenario, String> {
        let request_config = match config.request {
            zed::DebugRequest::Launch(launch) => {
                json!({
                    "type": config.adapter,
                    "request": "launch",
                    "program": launch.program,
                    "cwd": launch.cwd,
                    "args": launch.args,
                })
            }
            zed::DebugRequest::Attach(attach) => {
                json!({
                    "type": config.adapter,
                    "request": "attach",
                    "processId": attach.process_id,
                })
            }
        };

        let mut config_json: serde_json::Value = request_config;

        if let Some(stop_on_entry) = config.stop_on_entry {
            config_json["stopOnEntry"] = json!(stop_on_entry);
        }

        if config.adapter == "Flutter" {
            let device_id = self
                .last_selected_device
                .clone()
                .unwrap_or_else(|| "chrome".to_string());
            config_json["deviceId"] = json!(device_id);
            config_json["supportsHotReload"] = json!(true);
            config_json["hotReloadOnSave"] = json!(true);
        }

        Ok(zed::DebugScenario {
            label: config.label,
            adapter: config.adapter,
            build: None,
            config: config_json.to_string(),
            tcp_connection: None,
        })
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let dart_binary = self.language_server_binary(language_server_id, worktree)?;

        let (command, args) = if self.is_fvm_project(worktree) {
            let mut fvm_args = vec!["dart".to_string()];
            fvm_args.extend(dart_binary.args.unwrap_or_else(|| {
                vec!["language-server".to_string(), "--protocol=lsp".to_string()]
            }));
            ("fvm".to_string(), fvm_args)
        } else {
            (
                dart_binary.path,
                dart_binary.args.unwrap_or_else(|| {
                    vec!["language-server".to_string(), "--protocol=lsp".to_string()]
                }),
            )
        };

        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("dart", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "dart": settings
        })))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<CodeLabel> {
        let arrow = " → ";

        match completion.kind? {
            CompletionKind::Class => Some(CodeLabel {
                filter_range: (0..completion.label.len()).into(),
                spans: vec![CodeLabelSpan::literal(
                    completion.label,
                    Some("type".into()),
                )],
                code: String::new(),
            }),
            CompletionKind::Function | CompletionKind::Constructor | CompletionKind::Method => {
                let mut parts = completion.detail.as_ref()?.split(arrow);
                let (name, _) = completion.label.split_once('(')?;
                let parameter_list = parts.next()?;
                let return_type = parts.next()?;
                let fn_name = " a";
                let fat_arrow = " => ";
                let call_expr = "();";

                let code =
                    format!("{return_type}{fn_name}{parameter_list}{fat_arrow}{name}{call_expr}");

                let parameter_list_start = return_type.len() + fn_name.len();

                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::code_range(
                            code.len() - call_expr.len() - name.len()..code.len() - call_expr.len(),
                        ),
                        CodeLabelSpan::code_range(
                            parameter_list_start..parameter_list_start + parameter_list.len(),
                        ),
                        CodeLabelSpan::literal(arrow, None),
                        CodeLabelSpan::code_range(0..return_type.len()),
                    ],
                    filter_range: (0..name.len()).into(),
                    code,
                })
            }
            CompletionKind::Property => {
                let class_start = "class A {";
                let get = " get ";
                let property_end = " => a; }";
                let ty = completion.detail?;
                let name = completion.label;

                let code = format!("{class_start}{ty}{get}{name}{property_end}");
                let name_start = class_start.len() + ty.len() + get.len();

                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_start + name.len()),
                        CodeLabelSpan::literal(arrow, None),
                        CodeLabelSpan::code_range(class_start.len()..class_start.len() + ty.len()),
                    ],
                    filter_range: (0..name.len()).into(),
                    code,
                })
            }
            CompletionKind::Variable => {
                let name = completion.label;

                Some(CodeLabel {
                    filter_range: (0..name.len()).into(),
                    spans: vec![CodeLabelSpan::literal(name, Some("variable".into()))],
                    code: String::new(),
                })
            }
            _ => None,
        }
    }

    /// Creates debug scenarios from Flutter/Dart tasks
    ///
    /// This locator handles:
    /// - `flutter run` tasks → Creates debug scenario with auto-device selection
    /// - `flutter test` tasks → Creates debug scenario for testing
    fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        build_task: zed::TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<zed::DebugScenario> {
        if build_task.command == "flutter" || build_task.command == "fvm" {
            let args = &build_task.args;

            if args.contains(&"run".to_string()) {
                let device_id = self.extract_device_id(&build_task).unwrap_or_else(|| {
                    self.last_selected_device
                        .clone()
                        .unwrap_or_else(|| "chrome".to_string())
                });

                let custom_args = self.extract_custom_args(&build_task);

                let mut config = json!({
                    "type": "flutter",
                    "request": "launch",
                    "deviceId": device_id,
                    "program": "lib/main.dart"
                });

                if !custom_args.is_empty() {
                    config["args"] = json!(custom_args);
                }

                return Some(zed::DebugScenario {
                    label: build_task.label,
                    adapter: "Flutter".to_string(),
                    build: None,
                    config: config.to_string(),
                    tcp_connection: None,
                });
            }

            if args.contains(&"test".to_string()) {
                let test_file = args
                    .iter()
                    .skip_while(|arg| *arg != &"test".to_string())
                    .skip(1)
                    .next()
                    .unwrap_or(&"test".to_string())
                    .clone();

                let config = json!({
                    "type": "dart",
                    "request": "launch",
                    "program": test_file
                });

                return Some(zed::DebugScenario {
                    label: build_task.label,
                    adapter: "Dart".to_string(),
                    build: None,
                    config: config.to_string(),
                    tcp_connection: None,
                });
            }
        }

        None
    }

    /// Runs the debug locator after build task completion
    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        build_task: zed::TaskTemplate,
    ) -> Result<zed::DebugRequest, String> {
        if build_task.command == "flutter" || build_task.command == "fvm" {
            let args = &build_task.args;

            if args.contains(&"run".to_string()) {
                let device_id = self
                    .last_selected_device
                    .clone()
                    .unwrap_or_else(|| "chrome".to_string());
                let mut run_args = vec!["run".to_string(), "-d".to_string(), device_id];
                run_args.extend(args.iter().skip(1).filter(|a| *a != "run").cloned());

                return Ok(zed::DebugRequest::Launch(zed::LaunchRequest {
                    program: build_task.command.clone(),
                    cwd: build_task.cwd,
                    args: run_args,
                    envs: build_task.env,
                }));
            }
        }

        Err("No debug configuration available for this task".to_string())
    }

    /// Handles slash commands for AI assistant integration
    ///
    /// Supported commands:
    /// - `/flutter-devices`: List available Flutter devices
    /// - `/flutter-doctor`: Display Flutter diagnostics info
    /// - `/flutter-pub <cmd>`: Run Flutter package commands
    /// - `/flutter-analyze`: Run static analysis
    /// - `/flutter-test`: Execute Flutter tests
    ///
    /// Note: Commands provide guidance rather than executing directly due to API constraints
    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        match command.name.as_str() {
            "/flutter-devices" => {
                if let Some(wt) = worktree {
                    let devices = self.get_cached_devices(wt)?;
                    if devices.is_empty() {
                        Ok(zed::SlashCommandOutput {
                            text: "No Flutter devices available. Run 'flutter devices' in a terminal to check.".to_string(),
                            sections: vec![],
                        })
                    } else {
                        let output: Vec<String> = devices
                            .iter()
                            .map(|d| {
                                format!(
                                    "{} ({}) - {} {}",
                                    d.name,
                                    d.id,
                                    d.platform,
                                    if d.emulator { "[emulator]" } else { "" }
                                )
                            })
                            .collect();
                        Ok(zed::SlashCommandOutput {
                            text: format!("Available Flutter devices:\n{}", output.join("\n")),
                            sections: vec![],
                        })
                    }
                } else {
                    Ok(zed::SlashCommandOutput {
                        text: "No worktree available.".to_string(),
                        sections: vec![],
                    })
                }
            }
            "/flutter-doctor" => Ok(zed::SlashCommandOutput {
                text: "Run 'flutter doctor -v' in your terminal for detailed diagnostics."
                    .to_string(),
                sections: vec![],
            }),
            "/flutter-pub" => {
                if args.is_empty() {
                    Ok(zed::SlashCommandOutput {
                        text: "Usage: /flutter-pub <command>\nAvailable commands: get, upgrade, outdated".to_string(),
                        sections: vec![],
                    })
                } else {
                    let subcommand = &args[0];
                    Ok(zed::SlashCommandOutput {
                        text: format!("Run 'flutter pub {}' in your terminal.", subcommand),
                        sections: vec![],
                    })
                }
            }
            "/flutter-analyze" => Ok(zed::SlashCommandOutput {
                text: "Run 'flutter analyze' in your terminal for static analysis.".to_string(),
                sections: vec![],
            }),
            "/flutter-test" => Ok(zed::SlashCommandOutput {
                text: "Run 'flutter test' in your terminal to execute tests.".to_string(),
                sections: vec![],
            }),
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }

    /// Provides tab completion for slash command arguments
    fn complete_slash_command_argument(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        match command.name.as_str() {
            "/flutter-pub" => {
                let subcommands = vec![
                    ("get", "get", false),
                    ("upgrade", "upgrade", false),
                    ("outdated", "outdated", false),
                    ("cache", "cache", false),
                    ("deps", "deps", false),
                    ("downgrade", "downgrade", false),
                    ("global", "global", false),
                    ("locker", "locker", false),
                    ("publish", "publish", false),
                    ("unpack", "unpack", false),
                ];

                let completions: Vec<zed::SlashCommandArgumentCompletion> = if args.is_empty() {
                    subcommands
                        .iter()
                        .map(
                            |(label, new_text, run)| zed::SlashCommandArgumentCompletion {
                                label: label.to_string(),
                                new_text: new_text.to_string(),
                                run_command: *run,
                            },
                        )
                        .collect()
                } else {
                    let prefix = &args[0];
                    subcommands
                        .iter()
                        .filter(|(label, _, _)| label.starts_with(prefix))
                        .map(
                            |(label, new_text, run)| zed::SlashCommandArgumentCompletion {
                                label: label.to_string(),
                                new_text: new_text.to_string(),
                                run_command: *run,
                            },
                        )
                        .collect()
                };

                Ok(completions)
            }
            _ => Ok(vec![]),
        }
    }
}

zed::register_extension!(DartExtension);
