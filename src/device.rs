use zed_extension_api::{Result, Worktree};

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub emulator: bool,
}

pub fn get_cached_devices(
    device_cache: &[DeviceInfo],
    _worktree: &Worktree,
) -> Result<Vec<DeviceInfo>> {
    Ok(device_cache.to_vec())
}

pub fn select_best_device(
    device_cache: &[DeviceInfo],
    last_selected_device: &Option<String>,
    worktree: &Worktree,
) -> Result<String> {
    let devices = get_cached_devices(device_cache, worktree)?;

    if devices.is_empty() {
        return Ok("chrome".to_string());
    }

    if let Some(last_device_id) = last_selected_device {
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

pub fn ensure_device_available(
    device_id: &str,
    device_cache: &[DeviceInfo],
    worktree: &Worktree,
) -> Result<()> {
    let devices = get_cached_devices(device_cache, worktree)?;

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

pub fn get_platform_for_device(
    device_id: &str,
    device_cache: &[DeviceInfo],
    worktree: &Worktree,
) -> Result<String> {
    let devices = get_cached_devices(device_cache, worktree)?;

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

pub fn extract_device_id(task: &zed_extension_api::TaskTemplate) -> Option<String> {
    let args = &task.args;

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

pub fn extract_custom_args(task: &zed_extension_api::TaskTemplate) -> Vec<String> {
    let args = &task.args;
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
