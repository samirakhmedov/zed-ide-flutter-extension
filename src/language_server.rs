use zed_extension_api::lsp::CompletionKind;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{
    self as zed, serde_json, CodeLabel, CodeLabelSpan, LanguageServerId, Result, Worktree,
};

pub fn language_server_command(
    _language_server_id: &LanguageServerId,
    worktree: &Worktree,
) -> Result<zed::Command> {
    let default_args = vec!["language-server".to_string(), "--protocol=lsp".to_string()];

    // Check if user configured custom binary in settings
    if let Ok(Some(settings)) = get_custom_binary_settings(worktree) {
        return Ok(settings);
    }

    // Fallback: Use system dart from PATH
    if let Some(path) = worktree.which("dart") {
        return Ok(zed::Command {
            command: path,
            args: default_args,
            env: Default::default(),
        });
    }

    // Not found: show helpful error with manual configuration instructions
    Err("Dart SDK not found in PATH.\n\n\
         For FVM projects:\n\
         ────────────────────────────────────────────────────────────\n\
         Create .zed/settings.json in your project root:\n\n\
         {\n\
           \"lsp\": {\n\
             \"dart\": {\n\
               \"binary\": {\n\
                 \"path\": \"fvm\",\n\
                 \"arguments\": [\"dart\", \"language-server\"]\n\
               }\n\
             }\n\
           }\n\
         }\n\n\
         Then restart Zed.\n\n\
         For system Dart:\n\
         ────────────────────────────────────────────────────────────\n\
         • Install from dart.dev/get-dart\n\
         • Ensure 'dart' is in your PATH"
        .to_string())
}

fn get_custom_binary_settings(worktree: &Worktree) -> Result<Option<zed::Command>> {
    let settings = LspSettings::for_worktree("dart", worktree)
        .map_err(|e| format!("Failed to get LSP settings: {}", e))?;
    let binary = settings.binary.ok_or("Binary settings not configured")?;
    let path = binary.path.ok_or("Binary path not configured")?;

    Ok(Some(zed::Command {
        command: path,
        args: binary
            .arguments
            .unwrap_or_else(|| vec!["language-server".to_string(), "--protocol=lsp".to_string()]),
        env: Default::default(),
    }))
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
