use crate::{device, DartExtension};
use zed_extension_api::{Result, Worktree};

impl DartExtension {
    pub fn run_slash_command(
        &self,
        command: zed_extension_api::SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<zed_extension_api::SlashCommandOutput> {
        match command.name.as_str() {
            "/flutter-devices" => {
                if let Some(wt) = worktree {
                    let devices = device::get_cached_devices(&self.device_cache, wt)?;
                    if devices.is_empty() {
                        Ok(zed_extension_api::SlashCommandOutput {
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
                        Ok(zed_extension_api::SlashCommandOutput {
                            text: format!("Available Flutter devices:\n{}", output.join("\n")),
                            sections: vec![],
                        })
                    }
                } else {
                    Ok(zed_extension_api::SlashCommandOutput {
                        text: "No worktree available.".to_string(),
                        sections: vec![],
                    })
                }
            }
            "/flutter-doctor" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter doctor -v' in your terminal for detailed diagnostics."
                    .to_string(),
                sections: vec![],
            }),
            "/flutter-pub" => {
                if args.is_empty() {
                    Ok(zed_extension_api::SlashCommandOutput {
                        text: "Usage: /flutter-pub <command>\nAvailable commands: get, upgrade, outdated".to_string(),
                        sections: vec![],
                    })
                } else {
                    let subcommand = &args[0];
                    Ok(zed_extension_api::SlashCommandOutput {
                        text: format!("Run 'flutter pub {}' in your terminal.", subcommand),
                        sections: vec![],
                    })
                }
            }
            "/flutter-analyze" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter analyze' in your terminal for static analysis.".to_string(),
                sections: vec![],
            }),
            "/flutter-test" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter test' in your terminal to execute tests.".to_string(),
                sections: vec![],
            }),
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }

    pub fn complete_slash_command_argument(
        &self,
        command: zed_extension_api::SlashCommand,
        args: Vec<String>,
    ) -> Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>> {
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

                let completions: Vec<zed_extension_api::SlashCommandArgumentCompletion> =
                    if args.is_empty() {
                        subcommands
                            .iter()
                            .map(|(label, new_text, run)| {
                                zed_extension_api::SlashCommandArgumentCompletion {
                                    label: label.to_string(),
                                    new_text: new_text.to_string(),
                                    run_command: *run,
                                }
                            })
                            .collect()
                    } else {
                        let prefix = &args[0];
                        subcommands
                            .iter()
                            .filter(|(label, _, _)| label.starts_with(prefix))
                            .map(|(label, new_text, run)| {
                                zed_extension_api::SlashCommandArgumentCompletion {
                                    label: label.to_string(),
                                    new_text: new_text.to_string(),
                                    run_command: *run,
                                }
                            })
                            .collect()
                    };

                Ok(completions)
            }
            _ => Ok(vec![]),
        }
    }
}
