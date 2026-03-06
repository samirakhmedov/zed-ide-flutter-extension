# Device Management Guide

This guide explains how to manage Flutter devices in Zed for development and debugging.

## Overview

The Flutter extension for Zed provides intelligent device management to streamline your development workflow across mobile, web, and desktop platforms.

## Device Detection

### Automatic Device Detection

The extension automatically detects available Flutter devices including:
- **iOS Simulators** (macOS only)
- **Physical iOS Devices** (macOS only)
- **Android Emulators**
- **Physical Android Devices**
- **Chrome/Edge Browsers** (web)
- **Desktop Platforms** (macOS, Windows, Linux)

### Viewing Available Devices

**Method 1: Slash Command**
```
/flutter-devices
```

**Method 2: Task**
Run the "Flutter: Devices" task from the task picker (`Cmd+Shift+P` → "Tasks")

**Method 3: Terminal**
```bash
flutter devices
```

## Device Selection

### Automatic Selection

When you start debugging without specifying a device, the extension automatically selects the best device using these heuristics:

1. **Last used device** (if still available)
2. **Physical device** (non-emulator, non-web)
3. **Emulator/Simulator**
4. **Web browser** (Chrome)
5. **First available device**

### Manual Selection

Specify a device in your debug configuration:

```json
{
  "type": "flutter",
  "request": "launch",
  "deviceId": "chrome",
  "program": "lib/main.dart"
}
```

### Device IDs

Common device IDs:
- `chrome` - Chrome web browser
- `edge` - Edge web browser
- `macos` - macOS desktop
- `windows` - Windows desktop
- `linux` - Linux desktop
- Device-specific IDs (e.g., `iphone-15-pro`, `sdk-gph64-x86-64`)

## Device Caching

### How It Works

- Devices are cached for **5 minutes** to improve performance
- Cache is automatically refreshed when stale
- Cache persists across debug sessions

### Force Refresh

To force a device list refresh:
1. Run `/flutter-devices` slash command
2. Run `flutter devices` in terminal
3. Wait for cache to expire (5 minutes)

## Platform-Specific Notes

### iOS (macOS only)

**Simulators:**
- Must be started before detection
- Xcode must be installed
- Run `open -a Simulator` to launch

**Physical Devices:**
- Must be connected via USB or network
- Trust dialog must be accepted on device
- Developer mode must be enabled

### Android

**Emulators:**
- Android Studio must be installed
- Use "Flutter: Emulators" task to list/create emulators
- Start emulator before debugging

**Physical Devices:**
- USB debugging must be enabled
- Accept debugging prompt on device
- ADB must be in PATH

### Web

- Chrome or Edge browser required
- Automatically detected when installed
- No additional setup needed

### Desktop

- Platform-specific SDK required
- Build tools must be installed
- Works on respective platform only (macOS app on macOS, etc.)

## Troubleshooting

### No Devices Found

1. **Check Flutter installation:**
   ```bash
   flutter doctor -v
   ```

2. **Restart device/emulator**

3. **Refresh device list:**
   ```bash
   flutter devices
   ```

4. **Check platform requirements:**
   - iOS: Xcode installed?
   - Android: Android Studio installed?
   - Web: Chrome/Edge installed?

### Device Not Listed

1. Ensure device is running/emulator is started
2. Check platform-specific requirements (USB debugging, etc.)
3. Run `flutter devices` in terminal
4. Restart Zed

### Wrong Device Selected

Specify the device explicitly in debug configuration:
```json
{
  "deviceId": "your-device-id"
}
```

## Advanced Configuration

### Custom Device Selection

Create a debug configuration with specific device:

```json
{
  "label": "Flutter: Run on iPhone",
  "adapter": "Flutter",
  "type": "flutter",
  "request": "launch",
  "deviceId": "iphone-15-pro",
  "program": "lib/main.dart"
}
```

### Multiple Devices

To debug on multiple devices simultaneously:
1. Create separate debug configurations
2. Specify different `deviceId` for each
3. Start debug sessions independently

### Device-Specific Arguments

Pass device-specific arguments:

```json
{
  "type": "flutter",
  "request": "launch",
  "deviceId": "chrome",
  "program": "lib/main.dart",
  "args": ["--web-port=5000"]
}
```

## Best Practices

1. **Use physical devices** for accurate performance testing
2. **Use emulators** for quick iteration during development
3. **Use web** for fastest testing of UI changes
4. **Test on multiple platforms** before release
5. **Keep emulators running** to avoid startup delays
6. **Use `/flutter-devices`** to quickly check available devices

## Related Documentation

- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [Configuration Guide](./CONFIGURATION.md)
- [Keyboard Shortcuts](./KEYBOARD_SHORTCUTS.md)
- [Flutter DevTools](./DEVTOOLS_GUIDE.md)
