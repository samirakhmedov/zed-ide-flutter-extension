# Flutter Extension Setup Guide

This guide will help you configure the Flutter/Dart extension for Zed IDE.

## Quick Start

### For FVM Projects (Recommended)

If you're using FVM (Flutter Version Manager), you need to explicitly configure the SDK path.

**Step 1:** Create `.zed/settings.json` in your project root:

```json
{
  "lsp": {
    "dart": {
      "binary": {
        "path": "fvm",
        "arguments": ["dart", "language-server"]
      }
    }
  }
}
```

**Step 2:** Restart Zed

That's it! The extension will now use FVM's Dart SDK for your project.

### For System Dart/Flutter

If you're not using FVM and have Dart installed system-wide:

1. **Install Dart SDK:**
   - macOS: `brew install dart`
   - Windows: `choco install dart-sdk`
   - Linux: See [dart.dev/get-dart](https://dart.dev/get-dart)

2. **Verify installation:**
   ```bash
   which dart
   dart --version
   ```

3. **No configuration needed!** The extension will automatically use the system `dart` binary.

## Why Manual Configuration?

The extension uses **explicit configuration** rather than automatic detection because:

- ✅ **Works with any directory structure** - monorepos, multi-project workspaces, nested projects
- ✅ **Predictable behavior** - no mystery when detection fails
- ✅ **User control** - you decide which SDK to use
- ✅ **Zero performance overhead** - no filesystem scanning
- ✅ **Clear error messages** - you know exactly what to configure when something is wrong

## Advanced Configuration

### Multiple Flutter Projects with Different Versions

If you have multiple Flutter projects using different FVM versions:

```
workspace/
├── app1/
│   ├── .fvm/
│   ├── .zed/settings.json  ← Configured for FVM
│   └── pubspec.yaml
└── app2/
    ├── .fvm/
    ├── .zed/settings.json  ← Configured for FVM
    └── pubspec.yaml
```

Each project maintains its own `.zed/settings.json` with FVM configuration.

### Custom Dart SDK Path

If you have Dart installed at a custom location:

```json
{
  "lsp": {
    "dart": {
      "binary": {
        "path": "/custom/path/to/dart",
        "arguments": ["language-server", "--protocol=lsp"]
      }
    }
  }
}
```

### Debug Adapter Configuration

For debugging, you can also configure the debug adapter:

```json
{
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

### Backward Compatibility with `useFvm`

If you have existing debug configurations with the `useFvm: true` flag, they will continue to work:

```json
{
  "config": {
    "type": "flutter",
    "request": "launch",
    "program": "lib/main.dart",
    "useFvm": true  // This still works!
  }
}
```

However, we recommend migrating to the explicit binary configuration for clarity.

## Troubleshooting

### "Dart SDK not found in PATH"

**Solution:** Create `.zed/settings.json` with FVM configuration (see Quick Start above).

### "fvm command not found"

**Solution:** Install FVM:

```bash
dart pub global activate fvm
```

Then verify:

```bash
which fvm
fvm --version
```

### LSP still not working after configuration

1. **Verify JSON is valid:**
   ```bash
   cat .zed/settings.json | python -m json.tool
   ```

2. **Restart Zed completely** (not just reload)

3. **Check Zed log:**
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Windows/Linux)
   - Type "zed: open log"
   - Search for "dart" or errors

4. **Verify FVM is working:**
   ```bash
   cd /path/to/your/project
   fvm dart --version
   fvm flutter --version
   ```

### "Failed to start Dart language server"

This error means the configured binary path is invalid.

1. **Check the path in your settings.json:**
   - For FVM: `"path": "fvm"` (assuming `fvm` is in PATH)
   - For system Dart: Remove the configuration to use system `dart`

2. **Test the command manually:**
   ```bash
   # For FVM:
   fvm dart language-server --protocol=lsp
   
   # For system Dart:
   dart language-server --protocol=lsp
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

### Zed still using old configuration

1. Save all files
2. Close Zed completely
3. Reopen your project
4. If still old config, delete `.zed/work/` directory in your project

## Migration from Auto-Detection

If you were previously using the extension with automatic FVM detection:

**Before (unreliable):**
```
Project opens → Auto-detect .fvm/ → Maybe works? 🤷
```

**After (reliable):**
```
Project opens → Config exists → Always works ✅
```

### Migration Steps

1. **Update extension** to version 2.0.0 or later

2. **Create `.zed/settings.json`:**
   ```bash
   mkdir -p .zed
   cat > .zed/settings.json << 'EOF'
   {
     "lsp": {
       "dart": {
         "binary": {
           "path": "fvm",
           "arguments": ["dart", "language-server"]
         }
       }
     }
   }
   EOF
   ```

3. **Restart Zed**

Done! ✅

## FAQ

**Q: Do I need to configure this for every project?**  
A: Yes, but it's a one-time setup per project. You can commit `.zed/settings.json` to your repo so team members have the same configuration.

**Q: Can I use a global configuration instead of per-project?**  
A: You can add the configuration to your global Zed settings (`~/.config/zed/settings.json`), but per-project configuration is recommended for FVM projects since different projects may use different Flutter versions.

**Q: What if I don't use FVM?**  
A: No configuration needed! Just ensure `dart` is in your PATH, and the extension will use it automatically.

**Q: Can I still use the `useFvm` flag in debug configs?**  
A: Yes! Existing debug configurations with `useFvm: true` continue to work for backward compatibility.

**Q: Why did you remove auto-detection?**  
A: Auto-detection was unreliable and failed in various scenarios (monorepos, nested projects, certain directory structures). Manual configuration is explicit, predictable, and works in ALL scenarios.

## Getting Help

If you're still having issues:

1. Check the [GitHub Issues](https://github.com/samirakhmedov/zed-ide-flutter-extension/issues)
2. Review the Zed log (`zed: open log`)
3. Verify your FVM setup with `fvm doctor`
4. Create a new issue with:
   - Your FVM version (`fvm --version`)
   - Your Dart version (`dart --version`)
   - Your project structure
   - Contents of `.zed/settings.json`
   - Relevant Zed log entries
