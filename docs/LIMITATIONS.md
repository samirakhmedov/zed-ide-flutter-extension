# Known Limitations

This document outlines the current limitations of the Dart/Flutter extension for Zed and provides alternative approaches where available.

## Widget Inspector

**Status**: ❌ Not Available  
**Reason**: Requires custom panel/webview API not yet available in Zed

**Alternative**: Use Flutter DevTools
```bash
flutter pub global activate devtools
flutter pub global run devtools
```

DevTools provides:
- Widget inspector
- Widget tree visualization
- Property explorer
- Performance overlay

## Performance Overlay

**Status**: ❌ Not Available  
**Reason**: Requires custom overlay API not yet available in Zed

**Alternative**: Use Flutter DevTools Performance view
- Frame rendering analysis
- GPU usage metrics
- Performance timeline

## Visual Device Selector

**Status**: ⚠️ Limited  
**Reason**: No custom status bar API in Zed

**Current Approach**:
- Device selection via debug configuration
- Auto-selection with intelligent heuristics
- Manual selection via `device_id` in config

**Workaround**: Use `/flutter-devices` slash command in AI assistant to list devices

## Hot Reload Manual Trigger

**Status**: ⚠️ Limited  
**Current State**: Hot reload works on file save automatically

**Limitations**:
- No manual hot reload button/shortcut (planned)
- Hot restart on force-save not yet implemented
- Custom keyboard shortcuts not yet available

**Workaround**: Save file to trigger hot reload automatically

## Device Detection

**Status**: ⚠️ Limited  
**Reason**: Zed extension API doesn't provide shell command execution

**Current State**:
- Device list cached (not real-time)
- No automatic refresh when devices connect/disconnect
- Manual refresh requires restarting Zed

**Workaround**: Use `flutter devices` in terminal for real-time device list

## Task System FVM Integration

**Status**: ⚠️ Limited  
**Reason**: Zed task system uses static JSON configuration

**Current State**:
- Tasks are defined in `.zed/tasks.json` as static JSON
- No automatic command prefixing for FVM
- Users must explicitly use `fvm flutter` or `fvm dart` in task commands

**Workaround**: Use the task templates provided in [CONFIGURATION.md](./CONFIGURATION.md#task-templates) or [FLUTTER_TASKS.md](./FLUTTER_TASKS.md#task-templates). Copy the FVM templates to your `.zed/tasks.json`.

## Debug Locators

**Status**: ⚠️ Not Yet Implemented  
**Planned**: Yes (future release)

**Impact**:
- Cannot one-click debug from runnable indicators
- Requires manual debug configuration

**Workaround**: Use task picker to run tasks, or create debug configurations manually

## Enhanced Runnables

**Status**: ⚠️ Not Yet Implemented  
**Planned**: Yes (future release)

**Current State**:
- Basic test detection works
- No Flutter widget test detection
- No integration test detection

**Workaround**: Use task picker with file path

## Real-time FVM Status

**Status**: ⚠️ Limited  
**Reason**: No file system watcher API

**Current State**:
- FVM detected on extension load
- Status cached for session
- No automatic invalidation when `.fvm/` changes

**Workaround**: Restart Zed after FVM configuration changes

## Multi-root Workspace FVM

**Status**: ⚠️ Partial  
**Current State**:
- FVM detection works per worktree
- Each worktree independently managed

**Limitation**: Device selection shared across worktrees (not isolated)

## Custom Configuration UI

**Status**: ❌ Not Available  
**Reason**: No custom settings UI API

**Current Approach**: JSON configuration in `.zed/settings.json`

## Future Improvements

The following limitations are planned to be addressed in future releases:

### High Priority
1. ✅ FVM auto-detection and command routing
2. ✅ Auto-device selection
3. ⏳ Debug locators for one-click debugging
4. ⏳ Hot reload manual triggers
5. ⏳ Enhanced runnable detection

### Medium Priority
1. ⏳ Real-time device monitoring
2. ⏳ FVM cache invalidation
3. ⏳ Device preference persistence
4. ⏳ Custom keyboard shortcuts

### Low Priority (Requires Zed API Changes)
1. ❌ Widget Inspector
2. ❌ Performance Overlay
3. ❌ Custom Status Bar
4. ❌ Custom Settings UI

## Reporting Issues

If you encounter issues or have feature requests:

1. Check this limitations document
2. Search existing issues on GitHub
3. Create a new issue with:
   - Zed version
   - Extension version
   - Steps to reproduce
   - Expected vs actual behavior

## See Also

- [Flutter DevTools Documentation](https://docs.flutter.dev/development/tools/devtools/overview)
- [Zed Extension API Documentation](https://zed.dev/docs/extensions)
- [Troubleshooting Guide](./TROUBLESHOOTING.md) (when available)
