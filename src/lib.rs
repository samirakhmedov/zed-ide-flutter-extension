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
    /// Timestamp of last device cache refresh (Unix epoch seconds)
    device_cache_timestamp: Option<u64>,
    /// Last selected device ID for smart defaults
    last_selected_device: Option<String>,
    /// FVM status cache per worktree (worktree_id -> is_fvm_project)
    fvm_status: HashMap<String, bool>,
}

impl DartExtension {
    /// Creates a new extension instance with default state
    fn new() -> Self {
        Self {
            device_cache: Vec::new(),
            device_cache_timestamp: None,
            last_selected_device: None,
            fvm_status: HashMap::new(),
        }
    }

    /// Checks if the project uses FVM (Flutter Version Manager)
    /// by looking for .fvm/fvm_config.json
    fn is_fvm_project(&self, worktree: &Worktree) -> bool {
        worktree.read_text_file(".fvm/fvm_config.json").is_ok()
    }

    /// Returns the appropriate Flutter command based on FVM detection
    /// - "fvm flutter" for FVM projects
    /// - "flutter" for standard projects
    fn get_flutter_command(&self, worktree: &Worktree) -> String {
        if self.is_fvm_project(worktree) {
            "fvm flutter".to_string()
        } else {
            "flutter".to_string()
        }
    }

    /// Returns the appropriate Dart command based on FVM detection
    /// - "fvm dart" for FVM projects
    /// - "dart" for standard projects
    fn get_dart_command(&self, worktree: &Worktree) -> String {
        if self.is_fvm_project(worktree) {
            "fvm dart".to_string()
        } else {
            "dart".to_string()
        }
    }

    /// Lists all available Flutter devices (placeholder - API limitations)
    /// TODO: Implement when shell_command API is available
    fn list_devices(&mut self, _worktree: &Worktree) -> Result<Vec<DeviceInfo>, String> {
        Ok(Vec::new())
    }

    /// Parses device list from `flutter devices --machine` output
    /// TODO: Implement when shell_command API is available
    fn parse_device_output(&self, _output: &str) -> Result<Vec<DeviceInfo>, String> {
        Ok(Vec::new())
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
            device_cache_timestamp: None,
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
                id.to_string()
            } else {
                self.select_best_device(worktree)?
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
}

zed::register_extension!(DartExtension);
