# FVM Integration Guide

This guide explains how to use the Dart/Flutter extension with FVM (Flutter Version Manager) projects.

## What is FVM?

[FVM](https://fvm.app/) is a simple CLI to manage Flutter SDK versions per project. It allows you to:
- Use different Flutter versions for different projects
- Ensure consistent Flutter versions across development teams
- Easily switch between Flutter versions

## How FVM Integration Works

The Dart/Flutter extension for Zed **automatically detects** FVM-managed projects and adjusts commands accordingly.

### Automatic Detection

The extension checks for `.fvm/fvm_config.json` in your project root. If found, it automatically:
- Uses `fvm flutter` instead of `flutter`
- Uses `fvm dart` instead of `dart`
- Routes all Flutter/Dart commands through FVM

### Where FVM is Applied

| Feature | FVM Support | How It Works |
|---------|-------------|--------------|
| **Language Server** | ⚠️ Manual config | Requires explicit configuration in `.zed/settings.json` |
| **Debug Adapter** | ✅ Auto-detected | Uses `useFvm: true` flag or auto-detects FVM in PATH |
| **Slash Commands** | ✅ Auto-detected | Commands suggest `fvm flutter` when FVM is in PATH |
| **Tasks** | ❌ Manual only | Use `fvm` prefix explicitly in task commands |
| **Hot Reload** | ✅ Via debug adapter | Works when debugging with FVM |

> **Note:** Language server requires manual configuration. See the Setup Guide section below for instructions.

## Setup Guide

### 1. Install FVM

```bash
# Install FVM globally
dart pub global activate fvm

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

### 3. Verify Detection

The extension automatically detects FVM. To verify:

1. **Language Server**: Open a Dart file - the language server should start via FVM
2. **Debug Adapter**: Create a debug configuration (no special config needed)
3. **Console**: Check Zed's console for FVM-related messages

## Debug Configuration

### Basic Flutter Debug (FVM Auto-Detected)

```json
{
  "program": "lib/main.dart",
  "type": "flutter"
}
```

The extension will automatically use FVM if detected.

### Force FVM Usage

If auto-detection doesn't work, force FVM:

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "useFvm": true
}
```

### Disable FVM

To use global Flutter instead of FVM:

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "useFvm": false
}
```

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

4. The extension automatically uses the configured version

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

3. Restart Zed to refresh FVM detection

4. Run/debug your app with the new version

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

3. Zed extension uses the same version locally

## Troubleshooting

### FVM Not Detected

**Symptom**: Extension uses global Flutter instead of FVM

**Solutions**:
1. Verify `.fvm/fvm_config.json` exists in project root
2. Restart Zed to refresh detection
3. Add `"useFvm": true` to debug config

### FVM Command Not Found

**Symptom**: Error "FVM command not found"

**Solution**:
```bash
# Install FVM
dart pub global activate fvm

# Ensure FVM is in PATH
export PATH="$PATH":"$HOME/.pub-cache/bin"

# Verify
fvm --version
```

### Wrong Flutter Version

**Symptom**: Extension uses wrong Flutter version

**Solution**:
```bash
# Check configured version
fvm list

# Reinstall correct version
fvm install <version>
fvm use <version>

# Restart Zed
```

### Language Server Issues

**Symptom**: Language server not starting with FVM

**Solution**:
1. Check Zed's console for errors
2. Verify FVM is installed: `fvm --version`
3. Test manually: `fvm dart language-server`
4. Check `.fvm/fvm_config.json` is valid JSON

## Team Workflow

### Recommended FVM Workflow

1. **Project Setup**:
   ```bash
   # Create new Flutter project
   flutter create my_app
   cd my_app
   
   # Initialize FVM
   fvm install stable
   fvm use stable
   ```

2. **Version Control**:
   ```bash
   # Add FVM config to git
   git add .fvm/fvm_config.json
   git commit -m "Add FVM configuration"
   ```

3. **Team Onboarding**:
   ```bash
   # New team member setup
   git clone <repo>
   cd my_app
   
   # Install FVM
   dart pub global activate fvm
   
   # Install configured Flutter version
   fvm install
   
   # Get dependencies
   fvm flutter pub get
   ```

4. **Development**:
   - Open project in Zed
   - Extension auto-detects FVM
   - Code with correct Flutter version

5. **Version Updates**:
   ```bash
   # Update Flutter version
   fvm install newer_version
   fvm use newer_version
   
   # Restart Zed to refresh
   ```

### .gitignore for FVM

Add to your `.gitignore`:

```
# FVM
.fvm/flutter_sdk
```

**Note**: Keep `.fvm/fvm_config.json` in version control.

## Best Practices

### 1. Consistent Team Versions

Always commit `.fvm/fvm_config.json` to ensure all team members use the same Flutter version.

### 2. Update Flutter Version

```bash
# Install new version
fvm install 3.19.0

# Use in project
fvm use 3.19.0

# Test thoroughly
fvm flutter test
fvm flutter analyze

# Commit config change
git add .fvm/fvm_config.json
git commit -m "Update Flutter to 3.19.0"
```

### 3. Multiple Projects

Each project can have its own Flutter version:

```
project_a/
  .fvm/fvm_config.json  # Uses stable

project_b/
  .fvm/fvm_config.json  # Uses beta
```

### 4. Documentation

Add to your project's README:

```markdown
## Setup

1. Install FVM: `dart pub global activate fvm`
2. Install Flutter: `fvm install`
3. Get dependencies: `fvm flutter pub get`
4. Open in Zed (FVM auto-detected)
```

## FVM vs Global Flutter

| Feature | FVM | Global Flutter |
|---------|-----|----------------|
| Multiple versions | ✅ Yes | ❌ No |
| Per-project versions | ✅ Yes | ❌ No |
| Team consistency | ✅ Yes | ⚠️ Manual |
| Version switching | ✅ Easy | ⚠️ Reinstall |
| CI/CD integration | ✅ Built-in | ⚠️ Manual |

## Resources

- [FVM Official Documentation](https://fvm.app/)
- [FVM GitHub Repository](https://github.com/fluttertools/fvm)
- [Flutter Versioning Best Practices](https://docs.flutter.dev/release/upgrade)

## Support

If you encounter FVM-related issues:

1. Check this guide
2. Verify FVM installation: `fvm doctor`
3. Check Zed console for error messages
4. Try manual FVM command: `fvm flutter --version`
5. Report issues on the extension's GitHub repository
