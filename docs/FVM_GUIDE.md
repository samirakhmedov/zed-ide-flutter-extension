# FVM Integration Guide

This guide explains how to configure the Dart/Flutter extension for Zed with FVM (Flutter Version Manager).

## What is FVM?

[FVM](https://fvm.app/) is a CLI to manage Flutter SDK versions per project. It allows you to:
- Use different Flutter versions for different projects
- Ensure consistent Flutter versions across development teams
- Easily switch between Flutter versions

## Manual Configuration Required

This extension requires **explicit manual configuration** for FVM projects. There is no automatic detection - you must configure SDK paths in `.zed/settings.json`.

This approach provides:
- ✅ Predictable behavior in all scenarios
- ✅ Works with monorepos and nested projects
- ✅ Clear error messages when something is wrong
- ✅ Zero performance overhead from detection

## Quick Setup

### 1. Install FVM

```bash
# Install FVM globally
dart pub global activate fvm

# Ensure PATH includes pub-cache bin
export PATH="$PATH:$HOME/.pub-cache/bin"

# Verify installation
fvm --version
```

### 2. Configure Your Project

```bash
# Navigate to your Flutter project
cd my_flutter_project

# Install a Flutter SDK version
fvm install stable

# Use this version for the project
fvm use stable

# This creates .fvm/fvm_config.json
```

### 3. Configure Zed

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

### 4. Restart Zed

Close and reopen Zed to apply the configuration.

## Feature Support

| Feature | Configuration | Notes |
|---------|---------------|-------|
| **Language Server** | Manual `.zed/settings.json` | Required for FVM |
| **Debug Adapter** | Manual `.zed/settings.json` | Required for FVM |
| **Slash Commands** | Guidance only | Suggest both `flutter` and `fvm flutter` |
| **Tasks** | Manual | Use `fvm` prefix in task commands |
| **Hot Reload** | Via debug adapter | Works when properly configured |

## Common Scenarios

### Multiple Flutter Versions

**Scenario**: Team members use different Flutter versions

**Solution**:
1. Configure FVM in your project:
   ```bash
   fvm install 3.16.0
   fvm use 3.16.0
   ```

2. Commit `.fvm/fvm_config.json` to version control

3. Team members install FVM and run:
   ```bash
   fvm install
   fvm flutter pub get
   ```

4. Each team member creates `.zed/settings.json` with the FVM configuration

### Testing Different Flutter Versions

**Scenario**: Test your app with different Flutter versions

**Solution**:
1. Install multiple versions:
   ```bash
   fvm install stable
   fvm install beta
   fvm install 3.10.0
   ```

2. Switch versions:
   ```bash
   fvm use stable  # or beta, 3.10.0
   ```

3. Restart Zed to use the new version

### CI/CD Integration

**Scenario**: Ensure consistent Flutter version in CI/CD

**Solution**:
1. Your `.fvm/fvm_config.json` specifies the version
2. CI/CD script:
   ```bash
   # Install FVM
   dart pub global activate fvm
   
   # Install configured Flutter version
   fvm install
   
   # Use FVM commands
   fvm flutter pub get
   fvm flutter test
   fvm flutter build apk
   ```

## Task Configuration

For tasks in `.zed/tasks.json`, use `fvm` explicitly:

```json
[
  {
    "label": "Flutter: Run",
    "command": "fvm",
    "args": ["flutter", "run"],
    "tags": ["flutter-main"]
  },
  {
    "label": "Flutter: Pub Get",
    "command": "fvm",
    "args": ["flutter", "pub", "get"]
  },
  {
    "label": "Flutter: Test",
    "command": "fvm",
    "args": ["flutter", "test"]
  }
]
```

## Troubleshooting

### "Dart SDK not found" or "Flutter SDK not found"

**Solution**: Create `.zed/settings.json` with FVM configuration (see Quick Setup above).

### "fvm command not found"

**Solution**:
```bash
# Install FVM
dart pub global activate fvm

# Add to PATH (add to your shell profile for persistence)
export PATH="$PATH:$HOME/.pub-cache/bin"

# Verify
fvm --version
```

### LSP still not working after configuration

1. **Verify JSON is valid**:
   ```bash
   cat .zed/settings.json | python -m json.tool
   ```

2. **Restart Zed completely** (not just reload)

3. **Check Zed log**:
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)
   - Type "zed: open log"
   - Search for "dart" or errors

4. **Verify FVM is working**:
   ```bash
   cd /path/to/your/project
   fvm dart --version
   fvm flutter --version
   ```

### Multiple Flutter projects in workspace

Each Flutter project needs its own `.zed/settings.json` in the project root:

```
workspace/
├── project_a/
│   ├── .zed/settings.json  ← Configure for this project
│   └── pubspec.yaml
└── project_b/
    ├── .zed/settings.json  ← Configure for this project
    └── pubspec.yaml
```

## Quick Reference

### FVM Commands

```bash
# Install FVM
dart pub global activate fvm

# Install Flutter version
fvm install stable
fvm install 3.19.0

# Use version in project
fvm use stable

# List installed versions
fvm list

# Run Flutter commands
fvm flutter pub get
fvm flutter run
fvm flutter test
fvm flutter build apk

# Run Dart commands
fvm dart analyze
fvm dart test
```

### Zed Configuration Template

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

## Resources

- [FVM Official Documentation](https://fvm.app/)
- [FVM GitHub Repository](https://github.com/fluttertools/fvm)
- [Flutter Installation Guide](https://docs.flutter.dev/get-started/install)
- [Zed Extension Documentation](https://zed.dev/docs/extensions/developing-extensions)

## Getting Help

- Run `/fvm-install` in Zed Assistant for setup instructions
- Run `/flutter-install` in Zed Assistant for Flutter SDK installation guide
- Check the [GitHub Issues](https://github.com/samirakhmedov/zed-ide-flutter-extension/issues)
