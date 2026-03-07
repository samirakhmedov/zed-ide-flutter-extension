# Slash Commands Guide

Slash commands provide quick access to Flutter tools through Zed's AI assistant panel.

## FVM Detection

Slash commands automatically detect whether FVM (Flutter Version Manager) is available in your project's PATH and adjust suggestions accordingly.

### How It Works

When you run a slash command:
1. The extension checks if `fvm` is available in your worktree's PATH
2. If FVM is found, commands suggest `fvm flutter ...` instead of `flutter ...`
3. If FVM is not found, commands suggest standard `flutter ...` commands

### Output Examples

**With FVM detected:**
```
Run 'fvm flutter pub get' in your terminal.
```

**Without FVM:**
```
Run 'flutter pub get' in your terminal.
```

### When FVM Detection Applies

- ✅ `/flutter-devices` - Detects FVM
- ✅ `/flutter-doctor` - Detects FVM
- ✅ `/flutter-pub` - Detects FVM
- ✅ `/flutter-analyze` - Detects FVM
- ✅ `/flutter-test` - Detects FVM

## Available Commands

### /flutter-devices

**Purpose**: List available Flutter devices

**Usage**: Type `/flutter-devices` in the AI assistant panel

**Output Example**:
```
Available Flutter devices:
iPhone 15 Pro (iphone-15-pro) - ios [emulator]
Chrome (chrome) - web-javascript
Android SDK built for x86 (emulator-5554) - android [emulator]
```

**When to Use**:
- Before debugging to see available devices
- When device_id is not working
- To find the correct device ID for configuration

**Example Scenario**:
```
You: /flutter-devices
AI: [Lists devices]
You: I want to run on iPhone 15 Pro, what device_id should I use?
AI: Use "iphone-15-pro" as the device_id in your debug configuration.
```

### /flutter-doctor

**Purpose**: Display Flutter diagnostics information

**Usage**: Type `/flutter-doctor` in the AI assistant panel

**Output Example** (varies by FVM detection):
```
# With FVM in PATH:
Run 'fvm flutter doctor -v' in your terminal for detailed diagnostics.

# Without FVM:
Run 'flutter doctor -v' in your terminal for detailed diagnostics.
```

**When to Use**:
- Checking Flutter installation
- Diagnosing environment issues
- Before reporting bugs

**Example Scenario**:
```
You: /flutter-doctor
AI: Run 'flutter doctor -v' in your terminal for detailed diagnostics.
[You run the command in terminal]
You: I see an error about Android toolchain
AI: [Helps diagnose and fix the issue]
```

### /flutter-pub

**Purpose**: Run Flutter package management commands

**Usage**: `/flutter-pub <subcommand>`

**Subcommands**:
- `get` - Install dependencies
- `upgrade` - Upgrade dependencies
- `outdated` - Check for outdated packages

**Examples**:
```
/flutter-pub get
/flutter-pub upgrade
/flutter-pub outdated
```

**Output Example**:
```
Usage: /flutter-pub <command>
Available commands: get, upgrade, outdated
```

**When to Use**:
- Managing dependencies
- Updating packages
- Checking for updates

**Example Scenarios**:

1. **Install Dependencies**:
```
You: I added a new package to pubspec.yaml
AI: Run 'flutter pub get' in your terminal to install it.
You: /flutter-pub get
AI: Run 'flutter pub get' in your terminal.
```

2. **Check Updates**:
```
You: Are my packages up to date?
You: /flutter-pub outdated
AI: Run 'flutter pub outdated' in your terminal to check.
[You run command and see outdated packages]
You: How do I update them?
AI: Run 'flutter pub upgrade' to update all packages.
```

### /flutter-analyze

**Purpose**: Run Flutter static analysis

**Usage**: Type `/flutter-analyze` in the AI assistant panel

**Output**:
```
Run 'flutter analyze' in your terminal for static analysis.
```

**When to Use**:
- Before committing code
- Finding potential issues
- Code quality checks

**Example Scenario**:
```
You: I want to check my code for issues
You: /flutter-analyze
AI: Run 'flutter analyze' in your terminal for static analysis.
[You run command and see warnings]
You: How do I fix these warnings?
AI: [Provides suggestions for each warning]
```

### /flutter-test

**Purpose**: Run Flutter tests

**Usage**: Type `/flutter-test` in the AI assistant panel

**Output**:
```
Run 'flutter test' in your terminal to execute tests.
```

**When to Use**:
- Running all tests
- Before pushing changes
- CI/CD preparation

**Example Scenario**:
```
You: I want to run all my tests
You: /flutter-test
AI: Run 'flutter test' in your terminal to execute tests.
[You run command]
You: Some tests failed, can you help?
AI: [Analyzes test output and suggests fixes]
```

## Usage Patterns

### Pattern 1: Device Selection Workflow

```
You: /flutter-devices
AI: [Lists devices]
You: Which device should I use for web development?
AI: Use "chrome" for web development. Add this to your config:
    { "device_id": "chrome", "platform": "web" }
```

### Pattern 2: Dependency Management

```
You: I need to add the http package
AI: Add 'http: ^1.0.0' to your pubspec.yaml, then run:
    flutter pub get
You: /flutter-pub get
AI: Run 'flutter pub get' in your terminal.
```

### Pattern 3: Pre-Commit Checklist

```
You: I'm ready to commit my changes
AI: Let's run a pre-commit check:
    1. Run tests
    2. Run analysis
    3. Format code
You: /flutter-test
AI: Run 'flutter test' in your terminal.
[Tests pass]
You: /flutter-analyze
AI: Run 'flutter analyze' in your terminal.
[No issues]
You: All checks passed!
AI: Great! Now format your code:
    dart format .
```

## Integration with AI Assistant

### Context-Aware Help

The AI assistant can combine slash command output with contextual help:

```
You: /flutter-devices
AI: [Lists devices]
    You have 3 devices available. The iPhone 15 Pro simulator
    is ready. Would you like me to help you configure a debug
    setup for it?
You: Yes, please
AI: Create a debug configuration:
    {
      "type": "flutter",
      "program": "lib/main.dart",
      "device_id": "iphone-15-pro",
      "platform": "ios"
    }
```

### Multi-Step Workflows

```
You: I'm setting up a new Flutter project
AI: Let me help you set up:
    1. First, check your environment:
You: /flutter-doctor
AI: Run 'flutter doctor -v' in your terminal.
[You verify everything is green]
You: Environment looks good
AI: Great! Now let's check for devices:
You: /flutter-devices
AI: [Lists devices]
    You have Chrome available. Let's test your setup:
    1. Create a debug config with "chrome" as device_id
    2. Run the app to verify everything works
```

## Advanced Usage

### Combining Commands

Use multiple commands in sequence:

```
You: I want to update my project
AI: Let's update your project safely:
    1. Check for outdated packages
You: /flutter-pub outdated
AI: Run 'flutter pub outdated' in your terminal.
[You check what's outdated]
You: Looks safe to update
AI: 2. Upgrade packages
You: /flutter-pub upgrade
AI: Run 'flutter pub upgrade' in your terminal.
[You upgrade packages]
You: Upgraded!
AI: 3. Check for breaking changes
You: /flutter-analyze
AI: Run 'flutter analyze' in your terminal.
[No issues]
AI: 4. Run tests to verify
You: /flutter-test
AI: Run 'flutter test' in your terminal.
```

### Troubleshooting Workflows

```
You: My app isn't running
AI: Let's diagnose the issue:
    1. Check your environment
You: /flutter-doctor
AI: Run 'flutter doctor -v' in your terminal.
[Doctor shows issues]
You: Android toolchain has issues
AI: 2. Check available devices
You: /flutter-devices
AI: [Lists devices]
    The Android emulator isn't showing up. Let's fix the
    Android toolchain issue first.
```

## Limitations

### Current Constraints

1. **Command Execution**: Slash commands provide guidance, not direct execution
2. **Output Processing**: Terminal output not automatically analyzed (copy-paste to AI)
3. **Real-time Updates**: Device list not refreshed automatically

### FVM Detection

Slash commands automatically detect FVM and adjust command suggestions. See the [FVM Detection](#fvm-detection) section above for details.

### Workarounds

1. **Manual Execution**: Run suggested commands in terminal
2. **Share Output**: Copy terminal output to AI for analysis
3. **Refresh**: Restart Zed to refresh device cache

## Best Practices

### 1. Use for Quick Checks

Slash commands are fastest for:
- Checking available devices
- Quick diagnostics
- Getting command reminders

### 2. Combine with AI Assistant

Let AI help interpret results:
```
You: /flutter-analyze
[Run command in terminal]
You: [Paste output]
AI: You have 3 warnings. Here's how to fix them...
```

### 3. Pre-Commit Workflows

Use commands in sequence:
```
/flutter-test → /flutter-analyze → dart format .
```

### 4. Project Setup

Use commands during initial setup:
```
/flutter-doctor → /flutter-devices → /flutter-pub get
```

## Future Enhancements

Planned improvements:
- ✅ Basic slash commands (available)
- ⏳ Direct command execution (future)
- ⏳ Automatic output analysis (future)
- ⏳ Tab completion for arguments (planned)

## Examples by Scenario

### Scenario 1: New Project Setup

```
1. You: /flutter-doctor
   AI: Check environment

2. You: /flutter-devices
   AI: See available devices

3. You: /flutter-pub get
   AI: Install dependencies

4. AI: Your project is ready to run!
```

### Scenario 2: Dependency Update

```
1. You: I want to update my dependencies
   AI: Let's check what's outdated

2. You: /flutter-pub outdated
   AI: Run 'flutter pub outdated'

3. [Review outdated packages]

4. You: /flutter-pub upgrade
   AI: Run 'flutter pub upgrade'

5. You: /flutter-test
   AI: Verify tests still pass
```

### Scenario 3: Pre-Push Checklist

```
1. You: I'm ready to push my changes
   AI: Let's run pre-push checks

2. You: /flutter-test
   AI: Run tests

3. You: /flutter-analyze
   AI: Run analysis

4. You: All checks passed
   AI: Great! Push your changes
```

## See Also

- [Configuration Guide](./CONFIGURATION.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
- [Keyboard Shortcuts](./KEYBOARD_SHORTCUTS.md)
