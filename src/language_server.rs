use std::collections::HashMap;
use zed_extension_api::lsp::CompletionKind;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, serde_json, CodeLabel, CodeLabelSpan, LanguageServerId, Result, Worktree,
};

pub struct DartBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub fn language_server_binary(
    _language_server_id: &LanguageServerId,
    worktree: &Worktree,
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

pub fn language_server_command(
    language_server_id: &LanguageServerId,
    worktree: &Worktree,
    #[allow(unused_variables)] fvm_status: &mut HashMap<String, bool>,
) -> Result<zed::Command> {
    let dart_binary = language_server_binary(language_server_id, worktree)?;

    let (command, args) = {
        #[cfg(feature = "fvm-support")]
        {
            if crate::fvm::is_fvm_project(worktree, fvm_status) {
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
            }
        }
        #[cfg(not(feature = "fvm-support"))]
        {
            (
                dart_binary.path,
                dart_binary.args.unwrap_or_else(|| {
                    vec!["language-server".to_string(), "--protocol=lsp".to_string()]
                }),
            )
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
