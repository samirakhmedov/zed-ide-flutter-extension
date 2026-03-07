use zed_extension_api::serde_json::json;
use zed_extension_api::{
    self as zed, current_platform, serde_json, DebugAdapterBinary, DebugTaskDefinition, Os, Result,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

use crate::DartExtension;

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

        let tool_path = worktree.which(tool).ok_or_else(|| {
            let tool_name = if debug_mode == "flutter" {
                "Flutter"
            } else {
                "Dart"
            };
            let tool_lower = if debug_mode == "flutter" {
                "flutter"
            } else {
                "dart"
            };
            let tool_site = if debug_mode == "flutter" {
                "flutter.dev"
            } else {
                "dart.dev"
            };

            format!(
                "{} SDK not found.\n\n\
                 ═════════════════════════════════════════════════\n\
                 For FVM projects:\n\
                 ═════════════════════════════════════════════════\n\
                 Create .zed/settings.json in your project:\n\
                 {{\n\
                   \"lsp\": {{\n\
                     \"dart\": {{\n\
                       \"binary\": {{\n\
                         \"path\": \"fvm\",\n\
                         \"arguments\": [\"dart\", \"language-server\"]\n\
                       }}\n\
                     }}\n\
                   }},\n\
                   \"debug\": {{\n\
                     \"{}\": {{\n\
                       \"binary\": {{\n\
                         \"path\": \"fvm\",\n\
                         \"arguments\": [\"{}\", \"debug_adapter\"]\n\
                       }}\n\
                     }}\n\
                   }}\n\
                 }}\n\
                 \n\
                 ═════════════════════════════════════════════════\n\
                 For system {}:\n\
                 ═════════════════════════════════════════════════\n\
                 • Install from {}\n\
                 • Ensure '{}' is in your PATH\n\
                 \n\
                 Need help? Run /flutter-install or /fvm-install in Zed Assistant.",
                tool_name, debug_mode, tool_lower, tool_name, tool_site, tool_lower
            )
        })?;

        let (command, arguments) = (tool_path, vec!["debug_adapter".to_string()]);

        let device_id = user_config.get("device_id").and_then(|v| v.as_str());

        let platform = user_config.get("platform").and_then(|v| v.as_str());

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

        let mut config_map = serde_json::Map::new();
        config_map.insert("type".to_string(), json!(tool));
        config_map.insert("request".to_string(), json!(request));
        if let Some(uri) = vm_service_uri {
            config_map.insert("vmServiceUri".to_string(), json!(uri));
        }
        config_map.insert("program".to_string(), json!(program));
        config_map.insert("cwd".to_string(), json!(cwd.clone().unwrap_or_default()));
        config_map.insert("args".to_string(), json!(args));
        config_map.insert("stopOnEntry".to_string(), json!(false));

        if debug_mode == "flutter" {
            config_map.insert("flutterMode".to_string(), json!("debug"));
            if let Some(id) = device_id {
                config_map.insert("deviceId".to_string(), json!(id));
            }
            if let Some(p) = platform {
                config_map.insert("platform".to_string(), json!(p));
            }
            config_map.insert("supportsHotReload".to_string(), json!(supports_hot_reload));
            config_map.insert("hotReloadOnSave".to_string(), json!(hot_reload_on_save));
        }

        let config_json = serde_json::Value::Object(config_map).to_string();

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

    fn extract_device_id_from_args(args: &[String]) -> Option<String> {
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

    fn extract_custom_args_from_task(args: &[String]) -> Vec<String> {
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
                let device_id = Self::extract_device_id_from_args(args);
                let custom_args = Self::extract_custom_args_from_task(args);

                let mut config = json!({
                    "type": "flutter",
                    "request": "launch",
                    "program": "lib/main.dart"
                });

                if let Some(id) = device_id {
                    config["deviceId"] = json!(id);
                }

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
                let mut run_args = vec!["run".to_string()];

                // Check if device is already specified in args
                let has_device = args.iter().any(|arg| {
                    arg == "-d" || arg == "--device-id" || arg.starts_with("--device-id=")
                });

                if !has_device {
                    // Let Flutter auto-select device
                }

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
