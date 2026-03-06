# Keyboard Shortcuts Reference

## Flutter Development Shortcuts

> **Note**: Some shortcuts require additional implementation or Zed API support. See status for each shortcut.

### Hot Reload & Hot Restart

| Shortcut | Action | Status |
|----------|--------|--------|
| `Cmd+S` / `Ctrl+S` | Hot Reload (on save) | ✅ **Supported** - Automatically triggers hot reload when debugging Flutter and saving a file |
| `Cmd+Shift+S` / `Ctrl+Shift+S` | Hot Restart (on save) | ⏳ **Planned** - Will trigger hot restart when force-saving |
| `Cmd+R` / `Ctrl+R` | Hot Reload (manual) | ⏳ **Planned** - Manual hot reload command |
| `Cmd+Shift+R` / `Ctrl+Shift+R` | Hot Restart (manual) | ⏳ **Planned** - Manual hot restart command |

### Debugging

| Shortcut | Action | Status |
|----------|--------|--------|
| `F5` | Start Debugging | ✅ **Supported** - Standard Zed shortcut |
| `Shift+F5` | Stop Debugging | ✅ **Supported** - Standard Zed shortcut |
| `F9` | Toggle Breakpoint | ✅ **Supported** - Standard Zed shortcut |
| `F10` | Step Over | ✅ **Supported** - Standard Zed shortcut |
| `F11` | Step Into | ✅ **Supported** - Standard Zed shortcut |
| `Shift+F11` | Step Out | ✅ **Supported** - Standard Zed shortcut |

### Code Navigation

| Shortcut | Action | Status |
|----------|--------|--------|
| `Cmd+Click` / `Ctrl+Click` | Go to Definition | ✅ **Supported** - Via Dart LSP |
| `Cmd+Shift+O` / `Ctrl+Shift+O` | Go to Symbol | ✅ **Supported** - Via Dart LSP |
| `Cmd+P` / `Ctrl+P` | Quick Open | ✅ **Supported** - Standard Zed shortcut |

## Task Shortcuts (via Task Picker)

Access via `Cmd+Shift+P` / `Ctrl+Shift+P` → "task:" prefix

### Common Flutter Tasks

- **task: Flutter: Run** - Run Flutter app in debug mode
- **task: Flutter: Test File** - Run tests in current file
- **task: Flutter: Pub Get** - Install dependencies
- **task: Flutter: Doctor** - Check Flutter installation
- **task: Flutter: Devices** - List available devices
- **task: Flutter: Analyze** - Run static analysis

### Common Dart Tasks

- **task: Dart: Test File** - Run Dart tests in current file
- **task: Dart: Analyze** - Run Dart static analysis
- **task: Dart: Format** - Format Dart code

## Slash Commands (AI Assistant)

Access via AI Assistant panel:

- **/flutter-devices** - List available Flutter devices
- **/flutter-doctor** - Display Flutter diagnostics
- **/flutter-pub get** - Install Flutter packages
- **/flutter-pub upgrade** - Upgrade Flutter packages
- **/flutter-analyze** - Run static analysis
- **/flutter-test** - Run Flutter tests

## Configuration Shortcuts

### Debug Configuration

Create in `.zed/settings.json` or via debug panel:

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "hotReloadOnSave": true
}
```

### FVM Projects

For FVM-managed projects, enable automatic detection:

```json
{
  "program": "lib/main.dart",
  "type": "flutter",
  "useFvm": true
}
```

## Status Legend

- ✅ **Supported** - Feature is fully implemented and working
- ⏳ **Planned** - Feature is planned for future release
- ⚠️ **Limited** - Feature has limitations (see notes)

## Notes

1. **Hot Reload on Save**: Automatically enabled for Flutter debug configurations. Works with standard save (`Cmd+S` / `Ctrl+S`).

2. **Custom Keymaps**: Future versions will support custom keyboard shortcuts for extension commands via a `keymaps/` directory.

3. **Task Shortcuts**: All tasks are accessible via the task picker. Future versions may support direct keyboard shortcuts for common tasks.

4. **Platform Differences**:
   - macOS: Use `Cmd` key
   - Windows/Linux: Use `Ctrl` key

## See Also

- [Zed Debugging Documentation](https://zed.dev/docs/debugging)
- [Flutter Hot Reload Documentation](https://docs.flutter.dev/development/tools/hot-reload)
- [Dart LSP Capabilities](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)
