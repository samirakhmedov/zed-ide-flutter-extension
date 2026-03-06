use std::collections::HashMap;
use zed_extension_api::lsp::CompletionKind;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, serde_json, CodeLabel, CodeLabelSpan, LanguageServerId, Result, Worktree,
};

pub fn language_server_command(
    _language_server_id: &LanguageServerId,
    worktree: &Worktree,
    #[allow(unused_variables)] fvm_status: &mut HashMap<String, bool>,
) -> Result<zed::Command> {
    let default_args = vec!["language-server".to_string(), "--protocol=lsp".to_string()];

    let (command, args) = {
        #[cfg(feature = "fvm-support")]
        {
            let is_fvm = crate::fvm::is_fvm_project(worktree, fvm_status);

            if is_fvm {
                if worktree.which("fvm").is_none() {
                    return Err(
                        "FVM project detected (.fvm/fvm_config.json found) but 'fvm' command not found in PATH.\n\
                         Install FVM: dart pub global activate fvm\n\
                         Then restart Zed."
                            .to_string(),
                    );
                }

                let mut fvm_args = vec!["dart".to_string()];
                fvm_args.extend(default_args.clone());
                ("fvm".to_string(), fvm_args)
            } else if let Some(path) = worktree.which("dart") {
                (path, default_args)
            } else {
                return Err("Dart SDK not found in PATH.\n\
                     \n\
                     Options:\n\
                     1. Install Dart from dart.dev/get-dart\n\
                     2. Or configure FVM in your project: fvm use stable\n\
                     3. Or set LSP binary path in Zed settings"
                    .to_string());
            }
        }
        #[cfg(not(feature = "fvm-support"))]
        {
            if let Some(path) = worktree.which("dart") {
                (path, default_args)
            } else {
                return Err("Dart SDK not found in PATH.\n\
                     \n\
                     Install Dart from dart.dev/get-dart or set LSP binary path in Zed settings"
                    .to_string());
            }
        }
    };

    Ok(zed::Command {
        command,
        args,
        env: Default::default(),
    })
}

pub fn language_server_workspace_configuration(
    _language_server_id: &LanguageServerId,
    worktree: &Worktree,
) -> Result<Option<serde_json::Value>> {
    let settings = LspSettings::for_worktree("dart", worktree)
        .ok()
        .and_then(|lsp_settings| lsp_settings.settings.clone())
        .unwrap_or_default();

    Ok(Some(serde_json::json!({
        "dart": settings
    })))
}

pub fn label_for_completion(
    _language_server_id: &LanguageServerId,
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
