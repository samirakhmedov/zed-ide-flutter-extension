use std::collections::HashMap;
use zed_extension_api::Worktree;

pub fn is_fvm_project(worktree: &Worktree, fvm_status: &mut HashMap<String, bool>) -> bool {
    let worktree_id = worktree.root_path();

    if let Some(&is_fvm) = fvm_status.get(&worktree_id) {
        return is_fvm;
    }

    let is_fvm = worktree.read_text_file(".fvm/fvm_config.json").is_ok();
    fvm_status.insert(worktree_id, is_fvm);
    is_fvm
}
