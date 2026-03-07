# VS Code Flutter Extension Migration Guide

This guide helps you transition from VS Code's Flutter extension to Zed's Flutter extension.

## Overview

Zed provides a streamlined Flutter development experience with essential features for mobile, web, and desktop development. While some VS Code features are not yet available, Zed offers a fast, efficient workflow with core Flutter capabilities.

## Feature Comparison

### ✅ Available in Zed

| Feature | VS Code | Zed | Notes |
|---------|---------|-----|-------|
| **Language Server** | ✅ | ✅ | Full Dart analysis, completions, refactoring |
| **Debugging** | ✅ | ✅ | Full DAP support, breakpoints, variables |
| **Device Selection** | ✅ | ✅ | Auto-detection and selection |
| **Hot Reload** | ✅ | ⚠️ | Supported via debug config (manual trigger pending) |
| **Hot Restart** | ✅ | ⚠️ | Supported via debug config (manual trigger pending) |
| **Test Execution** | ✅ | ✅ | Run tests via tasks and runnables |
| **Flutter CLI** | ✅ | ✅ | All Flutter commands as tasks |
| **FVM Support** | ✅ | ✅ | Automatic FVM detection |
| **Multi-platform** | ✅ | ✅ | iOS, Android, Web, Desktop |
| **Slash Commands** | ✅ | ✅ | AI assistant integration |
| **Runnables** | ✅ | ✅ | Click-to-run tests and main() |
| **Code Formatting** | ✅ | ✅ | dart format integration |
| **Static Analysis** | ✅ | ✅ | dart analyze integration |

### ⚠️ Not Yet Available

| Feature | VS Code | Zed | Alternative |
|---------|---------|-----|-------------|
| **Widget Inspector** | ✅ | ❌ | Use Flutter DevTools |
| **Performance Overlay** | ✅ | ❌ | Use Flutter DevTools |
| **Device Dropdown** | ✅ | ❌ | Use tasks or debug config |
| **Debug Paint** | ✅ | ❌ | Use Flutter DevTools |
| **Repaint Rainbow** | ✅ | ❌ | Use Flutter DevTools |
| **Custom Panel UI** | ✅ | ❌ | Limited by Zed API |

## Configuration Migration

### Debug Configurations

**VS Code (launch.json):**
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Flutter: Run",
      "type": "dart",
      "request": "launch",
      "program": "lib/main.dart"
    }
  ]
}
```

**Zed (debug.json):**
```json
[
  {
    "label": "Flutter: Run",
    "adapter": "Flutter",
    "type": "flutter",
    "request": "launch",
    "program": "lib/main.dart"
  }
]
```

### FVM Configuration Migration (Breaking Change)

**⚠️ Important:** The `useFvm` debug configuration option has been removed in favor of explicit binary configuration.

**Old Approach (Deprecated):**
```json
{
  "config": {
    "type": "flutter",
    "program": "lib/main.dart",
    "useFvm": true
  }
}
```

**New Approach (Required):**

Create `.zed/settings.json` in your project root:

```json
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
```

**Why this change?**
- Explicit configuration is more reliable than automatic detection
- Works in all project structures (monorepos, nested projects)
- Clear error messages when configuration is wrong
- Zero performance overhead from detection logic

**Need help?** Run `/fvm-install` in Zed Assistant for complete setup instructions.

### Keybindings

| Action | VS Code | Zed |
|--------|---------|-----|
| **Debug Start** | `F5` | `F5` |
| **Debug Stop** | `Shift+F5` | `Shift+F5` |
| **Step Over** | `F10` | `F10` |
| **Step Into** | `F11` | `F11` |
| **Step Out** | `Shift+F11` | `Shift+F11` |
| **Toggle Breakpoint** | `F9` | `F9` |
| **Run Task** | `Cmd+Shift+P` | `Cmd+Shift+P` |

### Settings Migration

**VS Code (settings.json):**
```json
{
  "dart.lineLength": 80,
  "dart.enableSdkFormatter": true,
  "dart.previewHotReloadOnSaveWatcher": true
}
```

**Zed (settings.json):**
```json
{
  "lsp": {
    "dart": {
      "settings": {
        "dart": {
          "lineLength": 80
        }
      }
    }
  }
}
```

## Workflow Adaptation

### 1. Running Apps

**VS Code:**
- Select device from status bar
- Press `F5` or click "Run and Debug"

**Zed:**
- Use auto-detected device (or specify in config)
- Press `F5` or use "Flutter: Run" task
- Check devices with `/flutter-devices` slash command

### 2. Testing

**VS Code:**
- Click "Run" above test functions
- Use "Testing" panel

**Zed:**
- Click runnable indicator (▶️) in gutter
- Use test tasks from task picker
- Run specific tests via runnables

### 3. Package Management

**VS Code:**
- Right-click `pubspec.yaml` → "Get Packages"
- Use Command Palette

**Zed:**
- Run "Flutter: Pub Get" task
- Use `/flutter-pub get` slash command

### 4. Debugging

**VS Code:**
- Full debugging with widget inspector
- Visual debug tools

**Zed:**
- Core debugging (breakpoints, variables, call stack)
- Use Flutter DevTools for widget inspection
- Full DAP protocol support

### 5. Device Management

**VS Code:**
- Device selector in status bar
- Quick switch between devices

**Zed:**
- Auto-select best device
- Specify in debug config
- Use `/flutter-devices` to list available

## Hot Reload Workflow

### Current State

Hot reload flags are configured in debug sessions. Manual hot reload triggers may require additional setup:

1. **Save file** → Hot reload happens automatically (if configured)
2. **Manual hot reload** → Restart debug session (for now)

### Future Improvements

Hot reload keyboard shortcuts and commands are planned features.

## DevTools Integration

### When to Use DevTools

Use Flutter DevTools when you need features not yet in Zed:
- Widget Inspector
- Performance profiling
- Memory analysis
- Network inspection
- Logging viewer

### Launching DevTools

**Method 1: Task**
```
Run "Flutter: DevTools" task
```

**Method 2: Command Line**
```bash
flutter pub global run devtools
```

See [DevTools Guide](./DEVTOOLS_GUIDE.md) for details.

## FVM Projects

### VS Code Setup
```json
{
  "dart.flutterSdkPath": ".fvm/flutter_sdk"
}
```

### Zed Setup
No configuration needed! Zed auto-detects FVM and uses the correct Flutter version.

## Project Structure

Both editors support standard Flutter project structure:

```
my_flutter_app/
├── lib/
│   └── main.dart
├── test/
│   └── widget_test.dart
├── pubspec.yaml
├── .fvm/
│   └── fvm_config.json  (optional)
└── .zed/
    └── debug.json       (Zed debug configs)
```

## Tips for Smooth Transition

### 1. Start with Core Features
- Focus on debugging and running apps first
- Use tasks for common operations
- Get comfortable with keyboard shortcuts

### 2. Customize Your Setup
- Configure debug configurations for your workflow
- Set up frequently used tasks
- Adjust keybindings if needed

### 3. Use DevTools When Needed
- Widget inspection → Flutter DevTools
- Performance profiling → Flutter DevTools
- Advanced debugging → Flutter DevTools

### 4. Leverage Zed's Strengths
- Fast performance and quick startup
- Efficient task system
- AI assistant slash commands
- Clean, distraction-free interface

### 5. Keyboard-First Workflow
Zed is optimized for keyboard use:
- Use task picker (`Cmd+Shift+P`)
- Learn keyboard shortcuts
- Use runnables in gutter

## Common Tasks Comparison

### Create New Project

**VS Code:**
```
Cmd+Shift+P → "Flutter: New Project"
```

**Zed:**
```
Cmd+Shift+P → "Flutter: Create Project"
```

### Add Package

**VS Code:**
1. Edit `pubspec.yaml`
2. Auto-run `flutter pub get`

**Zed:**
1. Edit `pubspec.yaml`
2. Run "Flutter: Pub Get" task

### Run on Specific Device

**VS Code:**
1. Click device selector
2. Choose device
3. Press `F5`

**Zed:**
1. Create debug config with `"deviceId": "device-id"`
2. Press `F5`

Or use task arguments:
```bash
flutter run -d chrome
```

### Run Tests

**VS Code:**
1. Open test file
2. Click "Run" above test

**Zed:**
1. Open test file
2. Click runnable indicator (▶️)
3. Or use "Flutter: Test File" task

## Getting Help

### Zed Resources
- [Documentation](./README.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [Configuration Guide](./CONFIGURATION.md)

### Community
- GitHub Issues: Report bugs and request features
- Zed Community: Get help from other users

### Flutter Resources
- [Flutter Documentation](https://docs.flutter.dev)
- [Dart Documentation](https://dart.dev/guides)
- [Flutter DevTools](https://docs.flutter.dev/development/tools/devtools/overview)

## Summary

Zed provides essential Flutter development features with excellent performance. While some advanced VS Code features (like Widget Inspector) are not yet available, Zed's core functionality covers most development needs. Use Flutter DevTools for advanced features, and enjoy Zed's speed and efficiency for daily development.

## Next Steps

1. ✅ Configure your debug configurations
2. ✅ Set up FVM if using version management
3. ✅ Try the slash commands for AI integration
4. ✅ Explore the task system
5. ✅ Install DevTools for advanced features

Welcome to Flutter development in Zed! 🚀
