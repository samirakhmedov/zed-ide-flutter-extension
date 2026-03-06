# Hot Reload & Hot Restart Guide

This guide explains hot reload and hot restart functionality in Zed's Flutter extension.

## What is Hot Reload?

**Hot reload** allows you to see code changes instantly in your running Flutter app without losing app state. It injects updated source code into the Dart Virtual Machine (VM) while the app runs.

**Hot restart** reloads the entire app, resetting all state but applying all code changes since the last full restart.

## Current Implementation Status

### ✅ What's Working

- **Hot reload configuration** is included in debug sessions
- **Hot reload on save** can be enabled in debug configuration
- **Flutter debug adapter** supports hot reload protocol

### ⚠️ Limitations

- **Manual hot reload trigger** not yet available via keyboard shortcut
- **Hot reload button** not available in UI
- **Hot reload on save** requires configuration

### 🎯 Recommended Workflow

Since manual hot reload triggers are not yet implemented, use this workflow:

1. **Enable hot reload on save** (see configuration below)
2. **Save file** (`Cmd+S`) to trigger automatic hot reload
3. **See changes** instantly in running app
4. **Use hot restart** if state becomes corrupted

## Configuration

### Enable Hot Reload on Save

Add to your debug configuration:

```json
{
  "label": "Flutter: Run with Hot Reload",
  "adapter": "Flutter",
  "type": "flutter",
  "request": "launch",
  "program": "lib/main.dart",
  "supportsHotReload": true,
  "hotReloadOnSave": true
}
```

### Default Behavior

By default, Flutter debug sessions include:
```json
{
  "supportsHotReload": true,
  "hotReloadOnSave": true
}
```

## How Hot Reload Works

### Hot Reload Process

1. **You make code changes** in Zed
2. **Save the file** (`Cmd+S`)
3. **Zed sends changes** to Flutter debug adapter
4. **Debug adapter updates** Dart VM
5. **Flutter rebuilds widgets** with new code
6. **App updates instantly** (typically < 500ms)

### What Gets Reloaded

✅ **Hot Reload Updates:**
- Widget `build()` methods
- Helper functions
- Constants and variables
- Most method implementations

❌ **Requires Hot Restart:**
- Changes to `main()` method
- Global variable initializations
- Changes to app initialization code
- Native code changes

❌ **Requires Full Restart:**
- `pubspec.yaml` changes (add/remove packages)
- Native dependency changes
- Asset changes

## Hot Reload vs Hot Restart

| Feature | Hot Reload | Hot Restart | Full Restart |
|---------|------------|-------------|--------------|
| **Speed** | < 500ms | 2-5 seconds | 10+ seconds |
| **Preserves State** | ✅ Yes | ❌ No | ❌ No |
| **Code Changes** | Most | All | All |
| **Main() Changes** | ❌ | ✅ | ✅ |
| **Native Changes** | ❌ | ❌ | ✅ |

## Compatible Code Changes

### ✅ Hot Reload Compatible

**1. Widget UI Changes**
```dart
// Before
Text('Hello')

// After (hot reload works)
Text('Hello, World!')
```

**2. Logic Changes**
```dart
// Before
int calculate() => 1 + 1;

// After (hot reload works)
int calculate() => 2 + 2;
```

**3. Styling Changes**
```dart
// Before
Container(color: Colors.blue)

// After (hot reload works)
Container(color: Colors.red)
```

**4. Widget Tree Changes**
```dart
// Before
Column(children: [Text('A')])

// After (hot reload works)
Column(children: [Text('A'), Text('B')])
```

### ❌ Requires Hot Restart

**1. Main Method Changes**
```dart
// Changes to main() require hot restart
void main() {
  runApp(MyApp());
}
```

**2. Global State Initialization**
```dart
// Global variable changes require hot restart
final myGlobal = SomeExpensiveInitialization();
```

**3. Enum Changes**
```dart
// Adding/removing enum values requires hot restart
enum Status { loading, success, error }
```

### ❌ Requires Full Restart

**1. Package Dependencies**
```yaml
# Changes to pubspec.yaml require full restart
dependencies:
  flutter:
    sdk: flutter
  cupertino_icons: ^1.0.0
  new_package: ^2.0.0  # Added this
```

**2. Native Code Changes**
- iOS: Changes to `ios/` directory
- Android: Changes to `android/` directory
- Platform-specific code

**3. Asset Changes**
```yaml
# New assets require full restart
flutter:
  assets:
    - images/new_image.png  # Added this
```

## Troubleshooting

### Hot Reload Not Working

**Symptom:** Changes don't appear after saving

**Solutions:**
1. Check `hotReloadOnSave: true` in config
2. Verify debug session is active
3. Check for incompatible code changes (see above)
4. Try hot restart (restart debug session)
5. Check debug console for errors

### State Corruption

**Symptom:** App behaves unexpectedly after hot reload

**Solutions:**
1. Perform hot restart (restart debug session)
2. Check for stateful widget issues
3. Verify `setState()` is used correctly
4. Consider if state preservation is appropriate

### Hot Reload Errors

**Symptom:** "Hot reload rejected" or similar error

**Common Causes:**
- Type changes in global variables
- Changes to const variables
- Changes to class inheritance
- Generics type changes

**Solutions:**
1. Perform hot restart
2. If that fails, full restart
3. Check error message for specific reason

## Best Practices

### 1. Design for Hot Reload

**Stateless Widgets:**
```dart
// Better for hot reload
class MyWidget extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Text('Hello');
  }
}
```

**Stateful Widgets with Keys:**
```dart
// Keys help preserve state
class MyWidget extends StatefulWidget {
  final Key key;
  
  MyWidget({this.key}) : super(key: key);
  
  @override
  _MyWidgetState createState() => _MyWidgetState();
}
```

### 2. Use Development Mode

Hot reload only works in **debug mode** (default). It does not work in:
- Profile mode (`flutter run --profile`)
- Release mode (`flutter run --release`)

### 3. Test State Preservation

Before relying on hot reload:
1. Make UI changes
2. Navigate through app
3. Verify state is preserved
4. Check for unexpected behavior

### 4. Regular Full Restarts

Periodically perform full restarts to:
- Clear accumulated state
- Verify app works from scratch
- Catch issues hidden by hot reload

### 5. Use Breakpoints Wisely

Hot reload preserves:
- ✅ Breakpoints
- ✅ Debugger state
- ✅ Variable watches

Use breakpoints to inspect state after hot reload.

## Future Enhancements

### Planned Features

- ⏳ Keyboard shortcut for manual hot reload
- ⏳ Hot reload button in debug toolbar
- ⏳ Hot reload status indicator
- ⏳ Visual feedback for hot reload success/failure
- ⏳ Configurable hot reload delay

### Track Progress

Check GitHub issues for hot reload enhancement updates.

## Performance Considerations

### Hot Reload Speed

Typical hot reload times:
- **Simple UI change:** 200-500ms
- **Logic change:** 300-600ms
- **Multiple widgets:** 400-800ms
- **Complex state:** 500-1000ms

### Memory Usage

Frequent hot reloads may:
- Increase memory usage slightly
- Accumulate orphaned objects (rare)
- Usually self-correcting via GC

If memory becomes an issue:
1. Perform hot restart
2. If persists, full restart
3. Check for memory leaks in code

## Comparison with VS Code

| Feature | VS Code | Zed (Current) |
|---------|---------|---------------|
| **Hot Reload on Save** | ✅ | ✅ (via config) |
| **Manual Hot Reload Button** | ✅ | ⏳ Planned |
| **Hot Reload Keyboard Shortcut** | ✅ `Cmd+\` | ⏳ Planned |
| **Hot Restart Keyboard Shortcut** | ✅ `Cmd+Shift+\` | ⏳ Planned |
| **Hot Reload Status** | ✅ Status bar | ⏳ Planned |
| **Error Notifications** | ✅ Popup | ⏳ Planned |

## Debugging Hot Reload Issues

### Check Debug Console

Look for hot reload messages:
```
Performing hot reload...
Reloaded 1 of 435 libraries in 234ms.
```

### Enable Verbose Logging

Add to debug configuration:
```json
{
  "type": "flutter",
  "request": "launch",
  "program": "lib/main.dart",
  "supportsHotReload": true,
  "hotReloadOnSave": true,
  "verboseLogging": true
}
```

### Common Error Messages

**"Hot reload rejected"**
- Code change incompatible with hot reload
- Try hot restart

**"Failed to hot reload"**
- Debug connection lost
- Restart debug session

**"VM service disconnected"**
- App crashed or stopped
- Restart debug session

## Additional Resources

### Official Documentation
- [Flutter Hot Reload](https://docs.flutter.dev/development/tools/hot-reload)
- [Dart VM Service Protocol](https://github.com/dart-lang/sdk/blob/main/runtime/vm/service/service.md)

### Related Guides
- [Debugging Guide](./CONFIGURATION.md#debugging)
- [Troubleshooting](./TROUBLESHOOTING.md)
- [Migration from VS Code](./MIGRATION_GUIDE.md)

## Summary

Hot reload in Zed currently works via save-triggered updates. While manual triggers are not yet available, the core hot reload functionality is working and provides fast development iteration. For the best experience:

1. ✅ Enable `hotReloadOnSave: true`
2. ✅ Use hot reload for UI/logic changes
3. ✅ Use hot restart for state resets
4. ✅ Use full restart for dependency changes
5. ⏳ Stay tuned for keyboard shortcuts and UI improvements

Happy hot reloading! 🔥
