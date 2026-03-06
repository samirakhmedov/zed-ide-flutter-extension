# Flutter DevTools Integration Guide

Since Zed's extension API doesn't yet support custom panels and overlays, advanced Flutter development features are best accessed through **Flutter DevTools** - a powerful suite of debugging and performance tools provided by the Flutter team.

## What is Flutter DevTools?

Flutter DevTools is a web-based suite of debugging and performance tools for Flutter and Dart applications. It provides features that complement Zed's editing and basic debugging capabilities.

## Installation

### Option 1: Global Installation (Recommended)

```bash
# Install DevTools globally
flutter pub global activate devtools

# Launch DevTools server
flutter pub global run devtools
```

This will start a local web server (typically at `http://localhost:9100`) and provide a URL to connect to your running Flutter app.

### Option 2: Via Flutter Run

When running a Flutter app in debug mode, the console output includes a DevTools URL:

```bash
flutter run
# Look for: "Flutter run key commands" and "DevTools" URL
```

## Key Features

### 1. Widget Inspector

**What it does**: Visualize and explore your Flutter widget tree

**How to use**:
1. Run your Flutter app in debug mode
2. Open DevTools URL in browser
3. Navigate to "Inspector" tab
4. Click on widgets in the tree to inspect properties
5. Use "Select Widget" mode to click on the app UI

**Benefits**:
- Visual widget tree navigation
- Property inspection and modification
- Widget performance analysis
- Layout Explorer for understanding constraints

### 2. Performance View

**What it does**: Analyze your app's performance in real-time

**Features**:
- Frame rendering analysis
- GPU usage metrics
- Performance timeline
- Flutter frame chart
- CPU profiler

**How to use**:
1. Open DevTools while app is running
2. Navigate to "Performance" tab
3. Click "Record" to capture performance data
4. Interact with your app
5. Stop recording and analyze frames

### 3. Memory View

**What it does**: Track memory allocation and detect memory leaks

**Features**:
- Memory allocation timeline
- Heap snapshot analysis
- Memory leak detection
- Object inspection

**How to use**:
1. Open DevTools "Memory" tab
2. Take heap snapshots at different points
3. Compare snapshots to find memory leaks
4. Analyze object retention

### 4. Network View

**What it does**: Monitor HTTP requests and responses

**Features**:
- Request/response inspection
- Timing analysis
- Request filtering
- Response body preview

### 5. Logging View

**What it does**: Enhanced console logging with filtering

**Features**:
- Structured log viewing
- Log filtering and search
- Log level filtering
- Export logs

### 6. App Size Tool

**What it does**: Analyze your app's size and composition

**Features**:
- Size breakdown by category
- Diff analysis between builds
- Symbol size analysis

## Integration Workflow with Zed

### Recommended Development Flow

1. **Code & Edit**: Write Flutter code in Zed with full IDE support
   - Syntax highlighting
   - Auto-completion
   - Refactoring
   - Hot reload on save

2. **Debug Basic Issues**: Use Zed's debugger for:
   - Breakpoints
   - Variable inspection
   - Step-through debugging
   - Call stack navigation

3. **Advanced Analysis**: Use DevTools for:
   - Widget inspection
   - Performance profiling
   - Memory analysis
   - Network monitoring

### Example Session

```bash
# Terminal 1: Run your Flutter app
flutter run

# Terminal 2: Start DevTools
flutter pub global run devtools

# In Zed:
# - Edit code with hot reload
# - Set breakpoints
# - Debug basic issues

# In Browser (DevTools):
# - Inspect widget tree
# - Profile performance
# - Check memory usage
# - Monitor network calls
```

## DevTools URL Patterns

When your Flutter app is running in debug mode, DevTools URLs follow these patterns:

```
# Local development
http://localhost:9100?uri=http://localhost:xxxxx

# Android emulator
http://localhost:9100?uri=http://10.0.2.2:xxxxx

# Physical device
http://localhost:9100?uri=http://device-ip:xxxxx
```

## Tips for Best Experience

### Performance Profiling

1. **Profile in Profile Mode**: Use `flutter run --profile` for accurate performance data
2. **Record Short Sessions**: Record 10-30 second interactions for easier analysis
3. **Compare Baselines**: Record performance before and after changes

### Widget Inspector

1. **Use Select Widget Mode**: Click "Select Widget" button then tap UI elements
2. **Explore Layout**: Use Layout Explorer for complex layouts
3. **Check Repaints**: Enable "Show Repaint Rainbow" to see repaint frequency

### Memory Analysis

1. **Take Snapshots Regularly**: Snapshot before and after user flows
2. **Compare Snapshots**: Use diff view to find leaks
3. **Check Object Retention**: Understand why objects remain in memory

## Zed Task for DevTools

Add a custom task to launch DevTools easily:

```json
{
  "label": "Flutter: Launch DevTools",
  "command": "sh",
  "args": ["-c", "flutter pub global run devtools"],
  "tags": ["utility"]
}
```

Save this to your project's `.zed/tasks.json` file.

## Keyboard Shortcuts

While DevTools runs in your browser, you can:

1. **Browser Bookmark**: Bookmark your DevTools URL for quick access
2. **Browser Tab Pin**: Pin the DevTools tab to keep it accessible
3. **Zed Terminal**: Run DevTools in Zed's integrated terminal

## Advanced Features

### DevTools Extensions

DevTools supports extensions for additional functionality:
- Provider DevTools
- Bloc DevTools
- GetX DevTools

Install via `pub.dev` and they'll appear in DevTools automatically.

### Remote Debugging

Connect DevTools to remote devices:
1. Ensure device and dev machine on same network
2. Use device's IP address in VM service URI
3. Configure firewall to allow connections

## Troubleshooting

### DevTools Won't Connect

1. **Check VM Service**: Ensure app is running in debug mode
2. **Check Port**: Verify VM service port is accessible
3. **Firewall**: Allow connections on VM service port
4. **Network**: Ensure device and machine on same network

### Performance Issues

1. **Use Profile Mode**: Avoid debug mode for performance analysis
2. **Close Other Apps**: Reduce system load during profiling
3. **Shorter Recordings**: Easier to analyze shorter captures

## Resources

- [Flutter DevTools Official Documentation](https://docs.flutter.dev/development/tools/devtools/overview)
- [Flutter Performance Best Practices](https://docs.flutter.dev/performance)
- [Widget Inspector Deep Dive](https://docs.flutter.dev/development/tools/devtools/inspector)
- [Memory Profiling Guide](https://docs.flutter.dev/development/tools/devtools/memory)

## Future Integration

As Zed's extension API evolves, we plan to integrate DevTools more tightly:

- ✅ Hot reload on save (available)
- ⏳ Embedded DevTools panel (planned)
- ⏳ Inline performance hints (future)
- ⏳ Widget inspector sidebar (future)

Until then, the browser-based DevTools provides all the advanced features you need for professional Flutter development.
