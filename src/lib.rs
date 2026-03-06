#[cfg(any(feature = "fvm-support", feature = "language-server"))]
use std::collections::HashMap;

#[cfg(feature = "debug-adapter")]
mod debug_adapter;
#[cfg(feature = "fvm-support")]
mod fvm;
#[cfg(feature = "language-server")]
mod language_server;
#[cfg(feature = "slash-commands")]
mod slash_commands;

mod device;

pub use device::DeviceInfo;

#[cfg(all(feature = "language-server", not(feature = "debug-adapter")))]
use zed_extension_api::serde_json;
#[cfg(feature = "language-server")]
use zed_extension_api::LanguageServerId;
#[cfg(feature = "debug-adapter")]
use zed_extension_api::{
    serde_json, DebugAdapterBinary, DebugTaskDefinition, StartDebuggingRequestArgumentsRequest,
    Worktree,
};

struct DartExtension {
    device_cache: Vec<DeviceInfo>,
    last_selected_device: Option<String>,
    #[cfg(feature = "fvm-support")]
    fvm_status: HashMap<String, bool>,
}

impl zed_extension_api::Extension for DartExtension {
    fn new() -> Self {
        Self {
            device_cache: Vec::new(),
            last_selected_device: None,
            #[cfg(feature = "fvm-support")]
            fvm_status: HashMap::new(),
        }
    }

    #[cfg(feature = "debug-adapter")]
    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> zed_extension_api::Result<DebugAdapterBinary, String> {
        self.get_dap_binary(
            adapter_name,
            config,
            user_provided_debug_adapter_path,
            worktree,
        )
    }

    #[cfg(feature = "debug-adapter")]
    fn dap_request_kind(
        &mut self,
        adapter_name: String,
        config: serde_json::Value,
    ) -> zed_extension_api::Result<StartDebuggingRequestArgumentsRequest, String> {
        self.dap_request_kind(adapter_name, config)
    }

    #[cfg(feature = "debug-adapter")]
    fn dap_config_to_scenario(
        &mut self,
        config: zed_extension_api::DebugConfig,
    ) -> zed_extension_api::Result<zed_extension_api::DebugScenario, String> {
        self.dap_config_to_scenario(config)
    }

    #[cfg(feature = "language-server")]
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        #[cfg(feature = "fvm-support")]
        {
            language_server::language_server_command(
                language_server_id,
                worktree,
                &mut self.fvm_status,
            )
        }
        #[cfg(not(feature = "fvm-support"))]
        {
            let mut empty_fvm_status = HashMap::new();
            language_server::language_server_command(
                language_server_id,
                worktree,
                &mut empty_fvm_status,
            )
        }
    }

    #[cfg(feature = "language-server")]
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<serde_json::Value>> {
        language_server::language_server_workspace_configuration(language_server_id, worktree)
    }

    #[cfg(feature = "language-server")]
    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: zed_extension_api::lsp::Completion,
    ) -> Option<zed_extension_api::CodeLabel> {
        language_server::label_for_completion(language_server_id, completion)
    }

    #[cfg(feature = "debug-adapter")]
    fn dap_locator_create_scenario(
        &mut self,
        locator_name: String,
        build_task: zed_extension_api::TaskTemplate,
        resolved_label: String,
        debug_adapter_name: String,
    ) -> Option<zed_extension_api::DebugScenario> {
        self.dap_locator_create_scenario(
            locator_name,
            build_task,
            resolved_label,
            debug_adapter_name,
        )
    }

    #[cfg(feature = "debug-adapter")]
    fn run_dap_locator(
        &mut self,
        locator_name: String,
        build_task: zed_extension_api::TaskTemplate,
    ) -> zed_extension_api::Result<zed_extension_api::DebugRequest, String> {
        self.run_dap_locator(locator_name, build_task)
    }

    #[cfg(feature = "slash-commands")]
    fn run_slash_command(
        &self,
        command: zed_extension_api::SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> zed_extension_api::Result<zed_extension_api::SlashCommandOutput> {
        self.run_slash_command(command, args, worktree)
    }

    #[cfg(feature = "slash-commands")]
    fn complete_slash_command_argument(
        &self,
        command: zed_extension_api::SlashCommand,
        args: Vec<String>,
    ) -> zed_extension_api::Result<Vec<zed_extension_api::SlashCommandArgumentCompletion>> {
        self.complete_slash_command_argument(command, args)
    }
}

zed_extension_api::register_extension!(DartExtension);
