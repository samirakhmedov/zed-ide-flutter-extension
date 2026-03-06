# Zed Dart

A [Dart](https://dart.dev/) extension for [Zed](https://zed.dev) with comprehensive Flutter development support.

## Features

### Core Dart Support
- **Language Server**: Full Dart LSP integration with autocomplete, go-to-definition, and refactoring
- **Debug Adapter**: Debug Dart applications with breakpoints, variable inspection, and step-through debugging
- **Syntax Highlighting**: Tree-sitter based syntax highlighting for Dart

### Flutter Development
- **Flutter Debugging**: Debug Flutter applications on mobile, web, and desktop platforms
- **Device Management**: Automatic device detection and intelligent device selection
- **FVM Support**: Full Flutter Version Manager (FVM) integration for version-agnostic development
- **Hot Reload**: Save-triggered hot reload for rapid development (Flutter debug configurations)

### Command Tasks
- **Package Management**: `flutter pub get`, `flutter pub upgrade`, `flutter pub outdated`
- **Project Management**: `flutter clean`, `flutter create`, `flutter analyze`
- **Build Commands**: Multi-platform build support (APK, iOS, Web, macOS, Windows, Linux)
- **Diagnostics**: `flutter doctor`, `flutter devices`, `flutter emulators`

### Slash Commands (AI Assistant)
- `/flutter-devices`: List available Flutter devices
- `/flutter-doctor`: Display Flutter diagnostics
- `/flutter-pub`: Run Flutter package commands
- `/flutter-analyze`: Run static analysis
- `/flutter-test`: Execute Flutter tests

## Documentation

### User Guides
- **[Configuration Guide](./docs/CONFIGURATION.md)** - Complete configuration reference
- **[FVM Integration](./docs/FVM_GUIDE.md)** - Flutter Version Manager setup and usage
- **[Slash Commands](./docs/SLASH_COMMANDS.md)** - AI assistant command reference
- **[Keyboard Shortcuts](./docs/KEYBOARD_SHORTCUTS.md)** - Shortcuts and key bindings
- **[Troubleshooting](./docs/TROUBLESHOOTING.md)** - Common issues and solutions

### Advanced Topics
- **[Flutter DevTools](./docs/DEVTOOLS_GUIDE.md)** - Integration with Flutter DevTools
- **[Known Limitations](./docs/LIMITATIONS.md)** - Current constraints and alternatives
- **[Release Notes](./docs/RELEASE_NOTES.md)** - Version history and changes

### External Resources
- [Zed Dart Language Docs](https://zed.dev/docs/languages/dart)
- [Dart LSP Support Docs](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)

## Requirements

- **Dart SDK**: Install from [dart.dev](https://dart.dev/get-dart)
- **Flutter SDK** (for Flutter projects): Install from [flutter.dev](https://flutter.dev)
- **FVM** (optional): For Flutter version management, install via `dart pub global activate fvm`

## Configuration

### Basic Dart Debug Configuration

Create a debug configuration in your Zed settings:

```json
{
  "program": "lib/main.dart",
  "type": "dart"
}
```

### Flutter Debug Configuration

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "device_id": "chrome",
  "hotReloadOnSave": true
}
```

### FVM Projects

For FVM-managed projects, the extension automatically detects and uses FVM. You can also manually enable it:

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "useFvm": true
}
```

## Known Limitations

- **Widget Inspector**: Not available (requires custom panel API)
- **Performance Overlay**: Not available (requires custom overlay API)
- **Visual Device Selector**: Device selection via task picker/command palette only

For advanced Flutter development features, use [Flutter DevTools](https://docs.flutter.dev/development/tools/devtools/overview):

```bash
flutter pub global activate devtools
flutter pub global run devtools
```

## Acknowledgments

This extension was inspired by and builds upon the foundation of the **[official Zed Dart extension](https://github.com/zed-extensions/dart)**. 

The initial Dart language server integration and debug adapter implementation were derived from the official extension, with significant enhancements for Flutter development, including:

- FVM (Flutter Version Manager) integration
- Automatic device detection and selection
- Hot reload support
- Enhanced Flutter-specific debugging features
- Comprehensive task library
- AI assistant slash commands

I extend my gratitude to the original authors and contributors of the official Zed Dart extension:
- Abdullah Alsigar
- Flo
- ybbond
- nielsenko

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.
