use crate::DartExtension;
use zed_extension_api::{Result, Worktree};

impl DartExtension {
    pub fn run_slash_command(
        &self,
        command: zed_extension_api::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<zed_extension_api::SlashCommandOutput> {
        match command.name.as_str() {
            "/flutter-install" => Ok(zed_extension_api::SlashCommandOutput {
                text: r#"Flutter SDK Installation Guide
════════════════════════════════════════════════════════

1. Install Flutter SDK:
   • macOS:   brew install flutter
   • Windows: choco install flutter
   • Linux:   snap install flutter --classic
   • Manual:  https://docs.flutter.dev/get-started/install

2. Verify installation:
   flutter doctor

3. Configure Zed (if flutter not in PATH):
   Create .zed/settings.json in your project:
   {
     "lsp": {
       "dart": {
         "binary": {
           "path": "/path/to/flutter/bin/dart",
           "arguments": ["language-server"]
         }
       }
     },
     "debug": {
       "flutter": {
         "binary": {
           "path": "/path/to/flutter/bin/flutter",
           "arguments": ["debug_adapter"]
         }
       }
     }
   }

4. Restart Zed to apply configuration."#
                    .to_string(),
                sections: vec![],
            }),
            "/fvm-install" => Ok(zed_extension_api::SlashCommandOutput {
                text: r#"FVM Installation Guide
══════════════════════════════════════

1. Install FVM:
   dart pub global activate fvm

2. Ensure PATH includes pub-cache bin:
   export PATH="$PATH:$HOME/.pub-cache/bin"

3. Setup project:
   cd your_project
   fvm install stable
   fvm use stable

4. Configure Zed (.zed/settings.json):
   {
     "lsp": {
       "dart": {
         "binary": {
           "path": "fvm",
           "arguments": ["dart", "language-server"]
         }
       }
     },
     "debug": {
       "dart": {
         "binary": {
           "path": "fvm",
           "arguments": ["dart", "debug_adapter"]
         }
       },
       "flutter": {
         "binary": {
           "path": "fvm",
           "arguments": ["flutter", "debug_adapter"]
         }
       }
     }
   }

5. Verify:
   fvm flutter doctor

6. Restart Zed to apply configuration."#
                    .to_string(),
                sections: vec![],
            }),
            "/flutter-devices" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter devices' in your terminal to list available devices.\n\
                           For FVM projects: 'fvm flutter devices'"
                    .to_string(),
                sections: vec![],
            }),
            "/flutter-doctor" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter doctor -v' in your terminal for detailed diagnostics.\n\
                           For FVM projects: 'fvm flutter doctor -v'"
                    .to_string(),
                sections: vec![],
            }),
            "/flutter-pub" => {
                if args.is_empty() {
                    Ok(zed_extension_api::SlashCommandOutput {
                        text: "Usage: /flutter-pub <command>\n\
                               Available commands: get, upgrade, outdated\n\
                               Run 'flutter pub <command>' in your terminal.\n\
                               For FVM projects: 'fvm flutter pub <command>'"
                            .to_string(),
                        sections: vec![],
                    })
                } else {
                    let subcommand = &args[0];
                    Ok(zed_extension_api::SlashCommandOutput {
                        text: format!(
                            "Run 'flutter pub {}' in your terminal.\n\
                             For FVM projects: 'fvm flutter pub {}'",
                            subcommand, subcommand
                        ),
                        sections: vec![],
                    })
                }
            }
            "/flutter-analyze" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter analyze' in your terminal for static analysis.\n\
                           For FVM projects: 'fvm flutter analyze'"
                    .to_string(),
                sections: vec![],
            }),
            "/flutter-test" => Ok(zed_extension_api::SlashCommandOutput {
                text: "Run 'flutter test' in your terminal to execute tests.\n\
                           For FVM projects: 'fvm flutter test'"
                    .to_string(),
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
