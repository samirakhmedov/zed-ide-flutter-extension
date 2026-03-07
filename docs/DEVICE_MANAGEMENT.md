# Device Management Guide

This guide explains how to configure target devices for Flutter debugging in Zed.

## Overview

Flutter applications can run on multiple platforms: mobile (iOS, Android), web, and desktop. This extension allows you to specify which device to target when debugging.

## Device Selection

### Manual Selection (Recommended)

Specify the device explicitly in your debug configuration:

```json
{
  "config": {
    "type": "flutter",
    "request": "launch",
    "program": "lib/main.dart",
    "deviceId": "chrome"
  }
}
```

### Automatic Selection

If you don't specify `deviceId`, Flutter automatically selects the best available device:
1. Physical device (if connected)
2. Running emulator/simulator
3. Web browser (Chrome)

Flutter's built-in selection is intelligent and works well for most cases.

## Common Device IDs

| Platform | Device ID | Notes |
|----------|-----------|-------|
| **Web** | `chrome` | Chrome browser |
| **Web** | `edge` | Edge browser |
| **Web** | `web-server` | Headless web server |
| **Desktop** | `macos` | macOS desktop app |
| **Desktop** | `windows` | Windows desktop app |
| **Desktop** | `linux` | Linux desktop app |
| **Mobile** | Device-specific | Run `flutter devices` to list |

## Viewing Available Devices

### Method 1: Terminal
```bash
flutter devices
```

### Method 2: FVM Project
```bash
fvm flutter devices
```

### Method 3: Zed Assistant
```
/flutter-devices
```

## Configuration Examples

### Run on Chrome (Web)
```json
{
  "config": {
    "type": "flutter",
    "request": "launch",
    "program": "lib/main.dart",
    "deviceId": "chrome",
    "platform": "web"
  }
}
```

### Run on iOS Simulator
```json
{
  "config": {
    "type": "flutter",
    "request": "launch",
    "program": "lib/main.dart",
    "deviceId": "<simulator-id>",
    "platform": "ios"
  }
}
```

Run `flutter devices` to get the simulator ID.

### Run on Android Emulator
```json
{
  "config": {
    "type": "flutter",
    "request": "launch",
    "program": "lib/main.dart",
    "deviceId": "<emulator-id>",
    "platform": "android"
  }
}
```

Run `flutter devices` to get the emulator ID.

### Run on Physical Device

1. **Connect the device** via USB or network
2. **Enable developer mode** on the device
3. **Trust the computer** (iOS) or **enable USB debugging** (Android)
4. Run `flutter devices` to get the device ID
5. Add `deviceId` to your debug configuration

### Multiple Debug Configurations

Create separate configurations for different devices:

```json
[
  {
    "label": "Flutter: Chrome",
    "adapter": "Flutter",
    "config": {
      "type": "flutter",
      "request": "launch",
      "program": "lib/main.dart",
      "deviceId": "chrome"
    }
  },
  {
    "label": "Flutter: iOS",
    "adapter": "Flutter",
    "config": {
      "type": "flutter",
      "request": "launch",
      "program": "lib/main.dart",
      "deviceId": "iphone-15-pro"
    }
  },
  {
    "label": "Flutter: Android",
    "adapter": "Flutter",
    "config": {
      "type": "flutter",
      "request": "launch",
      "program": "lib/main.dart",
      "deviceId": "sdk-gph64-x86-64"
    }
  }
]
```

## Platform-Specific Notes

### iOS (macOS only)

**Simulators:**
- Must be started before debugging
- Xcode must be installed
- Run `open -a Simulator` to launch

**Physical Devices:**
- Connect via USB or network
- Trust dialog must be accepted
- Developer mode must be enabled

### Android

**Emulators:**
- Android Studio must be installed
- Start emulator before debugging

**Physical Devices:**
- Enable USB debugging
- Accept debugging prompt on device
- ADB must be in PATH

### Web

- Chrome or Edge browser required
- Automatically available when installed
- No additional setup needed

### Desktop

- Platform-specific SDK required
- Build tools must be installed
- Works on respective platform only

## Troubleshooting

### No Devices Found

1. **Check Flutter installation:**
   ```bash
   flutter doctor -v
   ```

2. **Start an emulator/simulator**

3. **Refresh device list:**
   ```bash
   flutter devices
   ```

4. **Check platform requirements:**
   - iOS: Xcode installed?
   - Android: Android Studio installed?
   - Web: Chrome/Edge installed?

### Device Not Listed

1. Ensure the device/emulator is running
2. Check platform-specific requirements
3. Run `flutter devices` to verify Flutter sees the device
4. Restart Zed

### Wrong Device Selected

Specify the device explicitly in your debug configuration:

```json
{
  "config": {
    "deviceId": "your-device-id"
  }
}
```

### Device Disconnects During Debug

This is a Flutter/Dart debug adapter limitation. If the device disconnects:
1. Stop the debug session
2. Reconnect the device or restart the emulator
3. Start a new debug session

## Best Practices

1. **Use explicit device IDs** in debug configurations for predictability
2. **Test on physical devices** for accurate performance
3. **Use emulators** for quick iteration during development
4. **Use web** for fastest testing of UI changes
5. **Test on multiple platforms** before release
6. **Keep emulators running** to avoid startup delays

## Related Documentation

- [Configuration Guide](./CONFIGURATION.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [Flutter Tasks Guide](./FLUTTER_TASKS.md)
