# Configuration Guide

This guide covers all configuration options for the Dart/Flutter extension in Zed.

## Debug Configuration

Debug configurations are defined in your Zed settings (`.zed/settings.json`) or via the debug panel.

### Flutter Debug Configuration

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "label": "Launch Flutter App",
  "device_id": "chrome",
  "platform": "web",
  "hotReloadOnSave": true,
  "args": [],
  "env": {}
}
```

### Dart Debug Configuration

```json
{
  "program": "bin/main.dart",
  "type": "dart",
  "label": "Launch Dart CLI",
  "args": [],
  "env": {}
}
```

## Configuration Properties

### Core Properties

| Property | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `type` | string | ✅ Yes | - | Debug type: `"flutter"` or `"dart"` |
| `program` | string | ✅ Yes | `"lib/main.dart"` | Entry point file path |
| `label` | string | ⚪ No | - | Human-readable configuration name |
| `request` | string | ⚪ No | `"launch"` | Request type: `"launch"` or `"attach"` |

### Flutter-Specific Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `device_id` | string | Auto-selected | Target device ID |
| `platform` | string | Auto-detected | Target platform: `web`, `ios`, `android`, `macos`, `windows`, `linux` |
| `hotReloadOnSave` | boolean | `true` | Enable hot reload on file save |
| `hotRestartOnSave` | boolean | `false` | Enable hot restart on force-save |
| `flutterMode` | string | `"debug"` | Build mode: `debug`, `profile`, `release` |

### FVM Configuration

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `useFvm` | boolean | Auto-detected | Force enable/disable FVM |

### Advanced Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `args` | array | `[]` | Command-line arguments |
| `env` | object | `{}` | Environment variables |
| `cwd` | string | Project root | Working directory |
| `vmServiceUri` | string | - | VM service URI for attach mode |
| `stopOnEntry` | boolean | `false` | Pause on first line |

## Device Selection

### Auto-Selection Priority

When `device_id` is not specified, the extension uses this priority:

1. **Last Used Device**: Previously selected device (if available)
2. **Physical Device**: Non-emulator, non-web device
3. **Emulator**: Android/iOS emulator
4. **First Available**: First device in the list
5. **Default**: `"chrome"` if no devices found

### Manual Device Selection

Specify a device explicitly:

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "device_id": "iphone-15-pro",
  "platform": "ios"
}
```

### Common Device IDs

| Platform | Device ID | Description |
|----------|-----------|-------------|
| Web | `chrome` | Google Chrome |
| Web | `edge` | Microsoft Edge |
| Web | `web-server` | Headless web server |
| iOS | `<device-id>` | iOS device/simulator ID |
| Android | `<device-id>` | Android device/emulator ID |
| Desktop | `macos` | macOS desktop |
| Desktop | `windows` | Windows desktop |
| Desktop | `linux` | Linux desktop |

To find device IDs:
```bash
flutter devices
```

## Platform Configuration

### Web (Chrome)

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "device_id": "chrome",
  "platform": "web"
}
```

### iOS

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "device_id": "iphone-15-pro",
  "platform": "ios"
}
```

### Android

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "device_id": "emulator-5554",
  "platform": "android"
}
```

### Desktop

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "device_id": "macos",
  "platform": "macos"
}
```

## FVM Projects

### Auto-Detection

FVM is automatically detected when `.fvm/fvm_config.json` exists:

```json
{
  "type": "flutter",
  "program": "lib/main.dart"
}
```

### Force FVM

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "useFvm": true
}
```

### Disable FVM

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "useFvm": false
}
```

## Attach Mode

Attach to a running Flutter/Dart application:

### Prerequisites

1. Run app with VM service enabled:
   ```bash
   flutter run --enable-vm-service
   ```

2. Note the VM service URI from console output:
   ```
   Flutter run listening on http://127.0.0.1:50342/...
   ```

### Configuration

```json
{
  "type": "flutter",
  "request": "attach",
  "vmServiceUri": "http://127.0.0.1:50342/...",
  "program": "lib/main.dart"
}
```

## Environment Variables

Add environment variables:

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "env": {
    "FLUTTER_ROOT": "/path/to/flutter",
    "DART_SDK": "/path/to/dart/sdk"
  }
}
```

## Command-Line Arguments

Pass arguments to the application:

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "args": [
    "--flavor",
    "development",
    "--dart-define=ENV=dev"
  ]
}
```

## Hot Reload Configuration

### Enable Hot Reload on Save

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "hotReloadOnSave": true
}
```

When you save a file (`Cmd+S` / `Ctrl+S`), the extension triggers hot reload.

### Disable Hot Reload

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "hotReloadOnSave": false
}
```

## Build Modes

### Debug Mode (Default)

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "flutterMode": "debug"
}
```

Features:
- Full debugging support
- Hot reload enabled
- Assertions enabled
- Larger app size

### Profile Mode

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "flutterMode": "profile"
}
```

Features:
- Performance profiling
- Limited debugging
- AOT compiled
- Near-release performance

### Release Mode

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "flutterMode": "release"
}
```

Features:
- No debugging
- Fully optimized
- AOT compiled
- Smallest size

## Advanced Scenarios

### Multiple Configurations

Create multiple configurations for different scenarios:

```json
[
  {
    "label": "Flutter: Chrome",
    "type": "flutter",
    "program": "lib/main.dart",
    "device_id": "chrome"
  },
  {
    "label": "Flutter: iOS",
    "type": "flutter",
    "program": "lib/main.dart",
    "device_id": "iphone-15-pro"
  },
  {
    "label": "Dart: CLI",
    "type": "dart",
    "program": "bin/cli.dart"
  }
]
```

### Conditional Configuration

Use environment-specific settings:

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "args": [
    "--flavor",
    "${env:FLUTTER_FLAVOR:development}"
  ]
}
```

### Custom Working Directory

```json
{
  "type": "flutter",
  "program": "lib/main.dart",
  "cwd": "./packages/my_app"
}
```

## Task Templates

Copy these templates to `.zed/tasks.json` in your project root. For FVM projects, use the FVM templates.

### FVM Task Templates

```json
[
  { "label": "Flutter: Run", "command": "fvm", "args": ["flutter", "run"], "tags": ["flutter-main"] },
  { "label": "Flutter: Pub Get", "command": "fvm", "args": ["flutter", "pub", "get"] },
  { "label": "Flutter: Analyze", "command": "fvm", "args": ["flutter", "analyze"] },
  { "label": "Flutter: Test", "command": "fvm", "args": ["flutter", "test"] },
  { "label": "Flutter: Test File", "command": "fvm", "args": ["flutter", "test", "$ZED_FILE"], "tags": ["flutter-test"] },
  { "label": "Flutter: Clean", "command": "fvm", "args": ["flutter", "clean"] },
  { "label": "Dart: Test File", "command": "fvm", "args": ["dart", "test", "$ZED_FILE"] }
]
```

### System Flutter Task Templates

```json
[
  { "label": "Flutter: Run", "command": "flutter", "args": ["run"], "tags": ["flutter-main"] },
  { "label": "Flutter: Pub Get", "command": "flutter", "args": ["pub", "get"] },
  { "label": "Flutter: Analyze", "command": "flutter", "args": ["analyze"] },
  { "label": "Flutter: Test", "command": "flutter", "args": ["test"] },
  { "label": "Flutter: Test File", "command": "flutter", "args": ["test", "$ZED_FILE"], "tags": ["flutter-test"] },
  { "label": "Flutter: Clean", "command": "flutter", "args": ["clean"] },
  { "label": "Dart: Test File", "command": "dart", "args": ["test", "$ZED_FILE"] }
]
```

## Debug Scenario Templates

Copy these templates to `.zed/debug.json` in your project root.

### FVM Debug Templates

```json
[
  { "label": "Flutter: Debug (Chrome)", "adapter": "Flutter", "request": "launch", "program": "lib/main.dart", "device_id": "chrome", "useFvm": true },
  { "label": "Flutter: Debug (iOS)", "adapter": "Flutter", "request": "launch", "program": "lib/main.dart", "useFvm": true },
  { "label": "Flutter: Debug (Android)", "adapter": "Flutter", "request": "launch", "program": "lib/main.dart", "useFvm": true },
  { "label": "Dart: Debug CLI", "adapter": "Dart", "request": "launch", "program": "bin/main.dart", "useFvm": true }
]
```

### System Flutter Debug Templates

```json
[
  { "label": "Flutter: Debug (Chrome)", "adapter": "Flutter", "request": "launch", "program": "lib/main.dart", "device_id": "chrome" },
  { "label": "Flutter: Debug (iOS)", "adapter": "Flutter", "request": "launch", "program": "lib/main.dart" },
  { "label": "Flutter: Debug (Android)", "adapter": "Flutter", "request": "launch", "program": "lib/main.dart" },
  { "label": "Dart: Debug CLI", "adapter": "Dart", "request": "launch", "program": "bin/main.dart" }
]
```

### Configuration Approaches for FVM

**Option 1: Using `useFvm` flag (recommended for debug configs):**
```json
{ "type": "flutter", "program": "lib/main.dart", "useFvm": true }
```

**Option 2: Using explicit binary configuration (for LSP/settings):**
```json
{
  "lsp": {
    "dart": {
      "binary": { "path": "fvm", "arguments": ["dart", "language-server"] }
    }
  }
}
```

## Troubleshooting

### Device Not Found

**Error**: "Device 'chrome' not found"

**Solution**:
1. Run `flutter devices` to list available devices
2. Update `device_id` in configuration
3. Or remove `device_id` to use auto-selection

### Flutter SDK Not Found

**Error**: "Flutter SDK not found"

**Solution**:
1. Install Flutter: https://flutter.dev
2. Add to PATH: `export PATH="$PATH:/path/to/flutter/bin"`
3. Verify: `flutter --version`

### FVM Not Working

**Error**: "FVM command not found"

**Solution**:
1. Install FVM: `dart pub global activate fvm`
2. Add to PATH: `export PATH="$PATH:$HOME/.pub-cache/bin"`
3. Verify: `fvm --version`

## Configuration Examples

### Minimal Flutter Config

```json
{
  "type": "flutter",
  "program": "lib/main.dart"
}
```

### Full Flutter Config

```json
{
  "label": "Flutter: Full Config",
  "type": "flutter",
  "program": "lib/main.dart",
  "request": "launch",
  "device_id": "chrome",
  "platform": "web",
  "useFvm": true,
  "flutterMode": "debug",
  "hotReloadOnSave": true,
  "stopOnEntry": false,
  "args": ["--flavor", "development"],
  "env": {
    "API_KEY": "your-api-key"
  },
  "cwd": "."
}
```

### Dart CLI Config

```json
{
  "label": "Dart: Run CLI",
  "type": "dart",
  "program": "bin/main.dart",
  "args": ["--verbose"],
  "env": {
    "LOG_LEVEL": "debug"
  }
}
```

## See Also

- [Debugging in Zed](https://zed.dev/docs/debugging)
- [Flutter Debugging Guide](https://docs.flutter.dev/testing/debugging)
- [FVM Integration Guide](./FVM_GUIDE.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md) (when available)
