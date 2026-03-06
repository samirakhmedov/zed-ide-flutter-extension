# Flutter Command Tasks

This document describes all available Flutter command tasks in Zed.

## Overview

Flutter tasks provide quick access to common Flutter CLI commands directly from Zed's task picker. All tasks support FVM (Flutter Version Manager) integration.

## Task Categories

### Development Tasks

#### Flutter: Pub Get
**Command:** `flutter pub get`  
**Purpose:** Download package dependencies  
**Shortcut:** Task picker  
**Tags:** `package-management`, `development`

```bash
# Equivalent to running:
flutter pub get
```

#### Flutter: Pub Upgrade
**Command:** `flutter pub upgrade`  
**Purpose:** Upgrade packages to latest versions  
**Tags:** `package-management`, `development`

```bash
# Equivalent to running:
flutter pub upgrade
```

#### Flutter: Pub Outdated
**Command:** `flutter pub outdated`  
**Purpose:** Check for outdated packages  
**Tags:** `package-management`, `development`

```bash
# Equivalent to running:
flutter pub outdated
```

#### Flutter: Clean
**Command:** `flutter clean`  
**Purpose:** Remove build artifacts  
**Tags:** `project-management`, `development`

```bash
# Equivalent to running:
flutter clean
```

#### Flutter: Analyze
**Command:** `flutter analyze`  
**Purpose:** Run static analysis  
**Tags:** `diagnostic`, `development`

```bash
# Equivalent to running:
flutter analyze
```

#### Flutter: Format
**Command:** `flutter format .`  
**Purpose:** Format Dart code  
**Tags:** `development`

```bash
# Equivalent to running:
flutter format .
```

### Build Tasks

#### Flutter: Build APK
**Command:** `flutter build apk`  
**Purpose:** Build Android APK  
**Tags:** `build`

```bash
# Equivalent to running:
flutter build apk
```

#### Flutter: Build iOS
**Command:** `flutter build ios`  
**Purpose:** Build iOS app (macOS only)  
**Tags:** `build`

```bash
# Equivalent to running:
flutter build ios
```

#### Flutter: Build Web
**Command:** `flutter build web`  
**Purpose:** Build web application  
**Tags:** `build`

```bash
# Equivalent to running:
flutter build web
```

#### Flutter: Build macOS
**Command:** `flutter build macos`  
**Purpose:** Build macOS desktop app  
**Tags:** `build`

```bash
# Equivalent to running:
flutter build macos
```

#### Flutter: Build Windows
**Command:** `flutter build windows`  
**Purpose:** Build Windows desktop app  
**Tags:** `build`

```bash
# Equivalent to running:
flutter build windows
```

#### Flutter: Build Linux
**Command:** `flutter build linux`  
**Purpose:** Build Linux desktop app  
**Tags:** `build`

```bash
# Equivalent to running:
flutter build linux
```

### Debug & Test Tasks

#### Flutter: Run
**Command:** `flutter run`  
**Purpose:** Run app with hot reload  
**Tags:** `flutter-main`, `debug`  
**Runnable:** Yes (detects `main()` in Flutter apps)

```bash
# Equivalent to running:
flutter run
```

#### Flutter: Test File
**Command:** `flutter test $ZED_FILE`  
**Purpose:** Run tests for current file  
**Tags:** `flutter-test-main`, `test`  
**Runnable:** Yes (detects test files)

```bash
# Equivalent to running:
flutter test path/to/test.dart
```

#### Flutter: Test All
**Command:** `flutter test`  
**Purpose:** Run all tests  
**Tags:** `test`

```bash
# Equivalent to running:
flutter test
```

### Diagnostic Tasks

#### Flutter: Doctor
**Command:** `flutter doctor -v`  
**Purpose:** Show Flutter environment diagnostics  
**Tags:** `diagnostic`, `utility`

```bash
# Equivalent to running:
flutter doctor -v
```

#### Flutter: Devices
**Command:** `flutter devices`  
**Purpose:** List available devices  
**Tags:** `device`, `utility`

```bash
# Equivalent to running:
flutter devices
```

#### Flutter: Emulators
**Command:** `flutter emulators`  
**Purpose:** List available emulators  
**Tags:** `device`, `utility`

```bash
# Equivalent to running:
flutter emulators
```

### Project Management Tasks

#### Flutter: Create Project
**Command:** `flutter create .`  
**Purpose:** Create new Flutter project in current directory  
**Tags:** `project-management`

```bash
# Equivalent to running:
flutter create .
```

#### Flutter: Upgrade
**Command:** `flutter upgrade`  
**Purpose:** Upgrade Flutter SDK  
**Tags:** `utility`

```bash
# Equivalent to running:
flutter upgrade
```

### Dart Tasks

#### Dart: Test File
**Command:** `dart test $ZED_FILE`  
**Purpose:** Run Dart tests for current file  
**Tags:** `dart-test-file`, `test`  
**Runnable:** Yes

```bash
# Equivalent to running:
dart test path/to/test.dart
```

#### Dart: Test Group
**Command:** `dart test "$ZED_FILE?line=$ZED_ROW"`  
**Purpose:** Run specific test group  
**Tags:** `dart-test-group`, `test`  
**Runnable:** Yes (click on test group)

```bash
# Equivalent to running:
dart test "path/to/test.dart?line=42"
```

#### Dart: Test Single
**Command:** `dart test "$ZED_FILE?line=$ZED_ROW"`  
**Purpose:** Run single test  
**Tags:** `dart-test-single`, `test`  
**Runnable:** Yes (click on individual test)

```bash
# Equivalent to running:
dart test "path/to/test.dart?line=42"
```

#### Dart: Analyze
**Command:** `dart analyze`  
**Purpose:** Run Dart static analysis  
**Tags:** `diagnostic`, `development`

```bash
# Equivalent to running:
dart analyze
```

#### Dart: Format
**Command:** `dart format .`  
**Purpose:** Format Dart code  
**Tags:** `development`

```bash
# Equivalent to running:
dart format .
```

## Task Variables

Tasks support Zed's built-in variables:

- `$ZED_FILE` - Current file path
- `$ZED_STEM` - Current filename without extension
- `$ZED_ROW` - Current line number
- `$ZED_COLUMN` - Current column number

## FVM Integration

All Flutter tasks automatically detect and use FVM (Flutter Version Manager) when available. If your project has `.fvm/fvm_config.json`, tasks will use `fvm flutter` instead of `flutter`.

## Running Tasks

### Method 1: Task Picker
1. Press `Cmd+Shift+P` (macOS) / `Ctrl+Shift+P` (Windows/Linux)
2. Type "flutter" or "dart"
3. Select desired task

### Method 2: Runnables
1. Open a Dart file
2. Look for the runnable indicator (▶️) in the gutter
3. Click to run the detected task

### Method 3: Keyboard Shortcut
- Assign keyboard shortcuts to frequently used tasks in your keymap

## Custom Tasks

You can create custom Flutter tasks in your project's `.zed/tasks.json`:

```json
[
  {
    "label": "Flutter: Run Release",
    "command": "flutter",
    "args": ["run", "--release"],
    "tags": ["custom"]
  },
  {
    "label": "Flutter: Build Split APK",
    "command": "flutter",
    "args": ["build", "apk", "--split-per-abi"],
    "tags": ["custom"]
  }
]
```

## Task Tags

Tasks are organized by tags for easy filtering:

- `debug` - Debug/run tasks
- `test` - Testing tasks
- `build` - Build tasks
- `development` - Development tasks
- `diagnostic` - Diagnostic tools
- `device` - Device management
- `package-management` - Package management
- `project-management` - Project management
- `utility` - Utility commands

## Tips & Best Practices

1. **Use runnables** for quick test execution
2. **Tag frequently used tasks** with custom tags
3. **Combine tasks** with debug configurations
4. **Run `flutter pub get`** after modifying `pubspec.yaml`
5. **Run `flutter clean`** when encountering build issues
6. **Use `flutter analyze`** before commits
7. **Check `flutter doctor`** if environment issues arise

## Related Documentation

- [Configuration Guide](./CONFIGURATION.md)
- [Keyboard Shortcuts](./KEYBOARD_SHORTCUTS.md)
- [Slash Commands](./SLASH_COMMANDS.md)
- [Troubleshooting](./TROUBLESHOOTING.md)
