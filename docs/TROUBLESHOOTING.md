# Troubleshooting Guide

This guide helps you resolve common issues with the Dart/Flutter extension for Zed.

## Quick Diagnosis

### Check Basic Setup

```bash
# Verify Flutter installation
flutter --version

# Verify Dart installation
dart --version

# Check Flutter doctor
flutter doctor -v

# Verify FVM (if using)
fvm --version
```

### Check Extension Status

1. Open Zed
2. Check for extension in sidebar
3. Open Zed console (View > Toggle Console)
4. Look for error messages

## Common Issues

### 1. Flutter SDK Not Found

**Symptom**: Error "Flutter SDK not found"

**Causes**:
- Flutter not installed
- Flutter not in PATH
- Manual configuration not set up

**Solutions**:

```bash
# Check if Flutter is in PATH
which flutter

# If not found, add to PATH
export PATH="$PATH:/path/to/flutter/bin"

# Add to shell profile for persistence
echo 'export PATH="$PATH:/path/to/flutter/bin"' >> ~/.zshrc  # or ~/.bashrc

# Verify
flutter --version
```

**For FVM projects**, create `.zed/settings.json`:

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
    "flutter": {
      "binary": {
        "path": "fvm",
        "arguments": ["flutter", "debug_adapter"]
      }
    }
  }
}
```

Need help? Run `/flutter-install` or `/fvm-install` in Zed Assistant.

### 2. Dart SDK Not Found

**Symptom**: Error "Dart SDK not found"

**Causes**:
- Dart not installed
- Dart not in PATH
- Manual configuration not set up

**Solutions**:

```bash
# Dart comes with Flutter
# Ensure Flutter is in PATH
flutter --version

# Use Flutter's Dart
export PATH="$PATH:/path/to/flutter/bin/cache/dart-sdk/bin"

# Verify
dart --version
```

**For FVM projects**, use manual configuration in `.zed/settings.json`:

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

### 3. FVM Command Not Found

**Symptom**: Error "'fvm' command not found"

**Causes**:
- FVM not installed
- FVM not in PATH

**Solutions**:

```bash
# Install FVM
dart pub global activate fvm

# Add pub cache to PATH
export PATH="$PATH:$HOME/.pub-cache/bin"

# Add to shell profile
echo 'export PATH="$PATH:$HOME/.pub-cache/bin"' >> ~/.zshrc

# Verify
fvm --version
```

After installing FVM, configure Zed by creating `.zed/settings.json`:

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

Run `/fvm-install` in Zed Assistant for complete setup instructions.

### 4. Device Not Found

**Symptom**: Error "Device 'chrome' not found" or similar

**Causes**:
- No devices connected
- Device not recognized
- Chrome/Edge not installed

**Solutions**:

```bash
# List available devices
flutter devices

# If no devices shown:
# For Chrome: Install Google Chrome
# For iOS: Open iOS Simulator
# For Android: Start Android emulator

# Restart Zed after connecting devices
```

**Auto-Selection**: Remove `device_id` from config to use automatic selection.

### 5. Hot Reload Not Working

**Symptom**: Hot reload doesn't trigger on save

**Causes**:
- Not in Flutter debug mode
- Hot reload disabled in config
- App not running

**Solutions**:

1. **Verify Debug Mode**: Ensure you're debugging (not just running)
2. **Check Configuration**:
   ```json
   {
     "type": "flutter",
     "program": "lib/main.dart",
     "hotReloadOnSave": true
   }
   ```
3. **Check Console**: Look for errors in Zed's debug console
4. **Manual Test**: Run `flutter run` in terminal to verify hot reload works

### 6. Language Server Not Starting

**Symptom**: No auto-completion, errors, or IDE features

**Causes**:
- Dart SDK not found
- Language server crashed
- Project not recognized

**Solutions**:

```bash
# Check Dart SDK
dart --version

# Test language server manually
dart language-server --protocol=lsp

# Check for .fvm if using FVM
ls -la .fvm/fvm_config.json

# Restart Zed
```

**Zed Console**: Check for language server errors in console (View > Toggle Console)

### 7. Debug Adapter Errors

**Symptom**: Debugging fails to start or connect

**Causes**:
- Flutter SDK version mismatch
- Incorrect configuration
- Port conflicts

**Solutions**:

1. **Check Flutter Version**:
   ```bash
   flutter --version
   flutter upgrade
   ```

2. **Verify Configuration**:
   ```json
   {
     "type": "flutter",
     "program": "lib/main.dart"
   }
   ```

3. **Check Port Availability**:
   ```bash
   # Kill processes on common ports
   lsof -ti:9100 | xargs kill -9  # DevTools
   lsof -ti:8080 | xargs kill -9  # Web server
   ```

### 8. Manual Configuration Required

**Symptom**: Extension uses wrong Flutter version or can't find SDK

**Causes**:
- No `.zed/settings.json` configured
- Incorrect binary path
- For FVM: not using `fvm` wrapper

**Solution**: Create `.zed/settings.json` in your project root

**For System Flutter (in PATH):**

No configuration needed if `flutter` is in your PATH. The extension will use it automatically.

**For FVM projects:**

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

**For custom Flutter paths:**

```json
{
  "lsp": {
    "dart": {
      "binary": {
        "path": "/path/to/flutter/bin/dart",
        "arguments": ["language-server"]
      }
    }
  },
  "debug": {
    "flutter": {
      "binary": {
        "path": "/path/to/flutter/bin/flutter",
        "arguments": ["debug_adapter"]
      }
    }
  }
}
```

```bash
# Restart Zed after creating config
```

**For FVM:**
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

**For custom Flutter path:**
```json
{
  "lsp": {
    "dart": {
      "binary": {
        "path": "/path/to/flutter/bin/dart",
        "arguments": ["language-server"]
      }
    }
  }
}
```

Need help? Run `/flutter-install` or `/fvm-install` in Zed Assistant.

### 9. Platform-Specific Issues

#### macOS: iOS Simulator Not Detected

```bash
# Open iOS Simulator
open -a Simulator

# List simulators
xcrun simctl list devices

# Boot a simulator
xcrun simctl boot "iPhone 15 Pro"
```

#### Windows: Android Emulator Issues

```bash
# List emulators
flutter emulators

# Launch emulator
flutter emulators --launch <emulator_id>

# Or use Android Studio's AVD Manager
```

#### Linux: Desktop Support Missing

```bash
# Enable desktop support
flutter config --enable-linux-desktop

# Rebuild
flutter create --platforms=linux .
```

### 10. Performance Issues

**Symptom**: Slow extension performance

**Causes**:
- Large project
- Too many devices
- Old Flutter version

**Solutions**:

1. **Update Flutter**:
   ```bash
   flutter upgrade
   ```

2. **Clear Cache**:
   ```bash
   flutter clean
   flutter pub get
   ```

3. **Reduce Devices**: Disconnect unused devices

4. **Check System Resources**: Close unnecessary apps

## Error Messages Reference

### "Failed to parse debug config"

**Cause**: Invalid JSON in debug configuration

**Solution**: Validate JSON syntax

```json
{
  "type": "flutter",
  "program": "lib/main.dart"  // No trailing comma
}
```

### "type is required and cannot be empty or null"

**Cause**: Missing or empty `type` field

**Solution**: Add type field

```json
{
  "type": "flutter",  // or "dart"
  "program": "lib/main.dart"
}
```

### "Program file not found"

**Cause**: Invalid `program` path

**Solution**: Use correct path

```json
{
  "type": "flutter",
  "program": "lib/main.dart"  // Relative to project root
}
```

### "Device X not found"

**Cause**: Specified device not available

**Solution**: List devices or use auto-selection

```bash
flutter devices
```

```json
{
  "type": "flutter",
  "program": "lib/main.dart"
  // Remove device_id for auto-selection
}
```

## Diagnostic Steps

### 1. Check Environment

```bash
# All-in-one check
echo "Flutter: $(flutter --version 2>&1 | head -1)"
echo "Dart: $(dart --version 2>&1 | head -1)"
echo "FVM: $(fvm --version 2>&1 || echo 'Not installed')"
echo "Devices: $(flutter devices 2>&1 | wc -l)"
```

### 2. Test Flutter App

```bash
# Create test app
flutter create test_app
cd test_app

# Test run
flutter run -d chrome

# If this works, issue is likely in extension config
```

### 3. Check Zed Console

1. Open Zed
2. View > Toggle Console
3. Look for errors related to:
   - Dart extension
   - Language server
   - Debug adapter

### 4. Verbose Logging

Enable verbose logging:

```bash
# Run Flutter with verbose output
flutter run -d chrome -v

# Check for errors
```

## Getting Help

### Before Asking

1. **Check This Guide**: Review common issues above
2. **Search Issues**: Check GitHub issues for similar problems
3. **Gather Info**:
   - Zed version
   - Extension version
   - Flutter version
   - OS version
   - Error messages
   - Steps to reproduce

### Where to Ask

1. **GitHub Issues**: For bugs and feature requests
2. **Zed Discord**: For general questions
3. **Flutter Community**: For Flutter-specific issues

### When Reporting

Include:

```markdown
**Environment**:
- Zed: v0.x.x
- Extension: v0.x.x
- Flutter: 3.x.x
- OS: macOS/Windows/Linux

**Description**:
What you expected vs what happened

**Steps to Reproduce**:
1. Step 1
2. Step 2
3. ...

**Configuration**:
```json
{
  "type": "flutter",
  "program": "lib/main.dart"
}
```

**Error Messages**:
```
Paste error messages here
```

**Additional Context**:
Screenshots, logs, etc.
```

## Prevention Tips

### 1. Keep Updated

```bash
# Update Zed
# (automatic for most installations)

# Update extension
# (via extension registry)

# Update Flutter
flutter upgrade

# Update FVM (if using)
dart pub global activate fvm
```

### 2. Valid Configuration

Always include required fields:

```json
{
  "type": "flutter",
  "program": "lib/main.dart"
}
```

### 3. Test Regularly

```bash
# Weekly checks
flutter doctor
flutter devices
```

### 4. Backup Configuration

Save working configurations to avoid re-creation.

## Related Resources

- [Configuration Guide](./CONFIGURATION.md)
- [FVM Integration Guide](./FVM_GUIDE.md)
- [Known Limitations](./LIMITATIONS.md)
- [Flutter Documentation](https://docs.flutter.dev)
- [Zed Documentation](https://zed.dev/docs)

## Still Need Help?

If this guide didn't solve your issue:

1. **Search GitHub Issues**: Your problem may already be reported
2. **Create New Issue**: Use the template above
3. **Community Help**: Ask on Zed Discord or Flutter community
