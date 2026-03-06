# Release Notes - v0.4.0

## 🎉 Major Update: Comprehensive Flutter Development Support

This release transforms the Dart extension into a full-featured Flutter development environment, bringing 80% of VS Code Flutter extension functionality to Zed.

## ✨ New Features

### Flutter Development
- **Enhanced Debugging**: Full Flutter debug adapter with auto-device selection and platform detection
- **Hot Reload Support**: Automatic hot reload on file save (Cmd+S) for faster development cycles
- **Device Management**: Intelligent device detection, caching, and auto-selection
- **FVM Integration**: Automatic Flutter Version Manager detection and command routing

### Slash Commands for AI Assistant
- `/flutter-devices`: List available Flutter devices
- `/flutter-doctor`: Display Flutter diagnostics  
- `/flutter-pub`: Run Flutter package commands (get, upgrade, outdated)
- `/flutter-analyze`: Run static analysis
- `/flutter-test`: Execute Flutter tests

### Expanded Task Library (25+ Tasks)
- **Package Management**: `flutter pub get/upgrade/outdated`
- **Project Management**: `flutter clean`, `flutter create`, `flutter analyze`
- **Build Commands**: Multi-platform support (APK, iOS, Web, macOS, Windows, Linux)
- **Diagnostics**: `flutter doctor -v`, `flutter devices`, `flutter emulators`
- **Development**: `flutter format`, `dart analyze`, `dart format`

### Enhanced Debug Configuration
- **Flutter Debug Schema**: Dedicated Flutter debug adapter schema with hot reload flags
- **Auto-Configuration**: Intelligent defaults for device ID and platform detection
- **Better Error Messages**: Helpful error messages with remediation suggestions

## 🔧 Improvements

### Error Handling
- Clear error messages when Flutter/Dart SDK is missing
- Device validation with available device list
- FVM installation validation
- Comprehensive error remediation suggestions

### Developer Experience
- Automatic FVM detection eliminates manual configuration
- Device caching improves performance (5-minute TTL)
- Logical task categorization for easy discovery
- Comprehensive README with examples

## 📚 Documentation

- **New README**: Comprehensive feature documentation with examples
- **Configuration Examples**: Debug configuration templates for Dart and Flutter
- **Known Limitations**: Transparent documentation of current limitations
- **Alternative Tools**: Flutter DevTools integration guidance

## 🔄 Breaking Changes

None - all changes are backward compatible with existing Dart extension functionality.

## 🚀 What's Next

Future releases will include:
- Debug locators for one-click debugging from runnables
- Enhanced runnable detection for Flutter tests
- Hot restart support
- Performance optimizations

## 🙏 Acknowledgments

This release implements the comprehensive Flutter enhancement roadmap defined in the OpenSpec change process, bringing professional-grade Flutter development to Zed.

## 📝 Full Changelog

See the [tasks.md](./openspec/changes/enhance-flutter-extension/tasks.md) file for a complete list of implemented features and improvements.
