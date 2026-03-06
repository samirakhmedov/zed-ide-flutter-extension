use zed_extension_api::serde_json::json;
use zed_extension_api::{
    self as zed, current_platform, serde_json, DebugAdapterBinary, DebugTaskDefinition, Os, Result,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

use crate::{device, DartExtension};

impl DartExtension {
    pub fn get_dap_binary(
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
            .unwrap_or_else(|| {
                #[cfg(feature = "fvm-support")]
                {
                    crate::fvm::is_fvm_project(worktree, &mut self.fvm_status)
                }
                #[cfg(not(feature = "fvm-support"))]
                {
                    false
                }
            });

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
                device::ensure_device_available(id, &self.device_cache, worktree)?;
                self.last_selected_device = Some(id.to_string());
                id.to_string()
            } else {
                let selected = device::select_best_device(
                    &self.device_cache,
                    &self.last_selected_device,
                    worktree,
                )?;
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
                device::get_platform_for_device(&device_id, &self.device_cache, worktree)?
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

    pub fn dap_request_kind(
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

    pub fn dap_config_to_scenario(
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

    pub fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        build_task: zed::TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<zed::DebugScenario> {
        if build_task.command == "flutter" || build_task.command == "fvm" {
            let args = &build_task.args;

            if args.contains(&"run".to_string()) {
                let device_id = device::extract_device_id(&build_task).unwrap_or_else(|| {
                    self.last_selected_device
                        .clone()
                        .unwrap_or_else(|| "chrome".to_string())
                });

                let custom_args = device::extract_custom_args(&build_task);

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
                    .nth(1)
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

    pub fn run_dap_locator(
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
}
