# AGENTS.md

Essential information for agentic coding agents working in this repository.

## Project Overview

**Zed extension for Dart/Flutter development** written in Rust. Provides:
- Dart LSP integration
- Debug adapter for Dart/Flutter
- Flutter device management & hot reload
- Manual SDK configuration (FVM or custom paths)
- AI assistant slash commands

**Repository**: https://github.com/samirakhmedov/zed-ide-flutter-extension

## Build, Lint & Test Commands

```bash
# Build extension (creates .wasm in target/wasm32-unknown-unknown/release/)
cargo build --release

# Type check
cargo check

# Format code (MUST run before commits)
cargo fmt

# Run linter (MUST pass before commits)
cargo clippy --all-targets --all-features -- -D warnings

# Combined check (recommended before commits)
cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings

# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with verbose output
cargo test -- --nocapture
```

### Feature Flags

Build with different feature combinations:

```bash
# Default build (all features: debug-adapter, language-server, slash-commands)
cargo build --release

# Minimal build (LSP only)
cargo build --release --no-default-features --features language-server

# Debug adapter only
cargo build --release --no-default-features --features debug-adapter

# Without FVM support
cargo build --release --no-default-features --features debug-adapter,language-server,slash-commands
```

Available features:
- `debug-adapter`: Debug adapter protocol implementation
- `language-server`: LSP integration
- `slash-commands`: AI assistant commands (`/flutter-*`)

Note: `fvm-support` feature has been removed. FVM is now supported via manual configuration.

## Project Structure

**Modular architecture** (decomposed from single file):

```
src/
├── lib.rs                  (165 lines) - Core extension, state management, registration
├── debug_adapter.rs        (356 lines) - DAP implementation (get_dap_binary, scenarios)
├── language_server.rs      (176 lines) - LSP binary setup, workspace config, completions
├── device.rs               (144 lines) - Device selection, caching, platform detection
└── slash_commands.rs       (128 lines) - `/flutter-*` command handlers
```

Note: `src/fvm.rs` has been removed. FVM detection replaced with manual configuration.

Other key files:
- `extension.toml`: Extension metadata and configuration
- `Cargo.toml`: Rust package configuration with feature flags
- `docs/`: User-facing documentation
- `debug_adapter_schemas/`: JSON schemas for debug adapters
- `languages/`: Language configuration files

## Code Style Guidelines

### Formatting & Naming

1. **Format**: Use `cargo fmt` (max line width: 100 chars, 4-space indentation)
2. **Naming conventions**:
   - Types/Structs/Enums: `PascalCase` (`DeviceInfo`, `DartExtension`)
   - Functions/Methods: `snake_case` (`get_cached_devices`, `is_fvm_project`)
   - Variables: `snake_case` (`device_id`, `worktree_id`)
   - Constants: `SCREAMING_SNAKE_CASE` (`MAX_DEVICES`)
   - Modules: `snake_case` (`mod device_management`)

### Imports Organization

```rust
// 1. Standard library
use std::collections::HashMap;

// 2. External crates
use zed_extension_api::serde_json::json;
use zed_extension_api::{
    self as zed, current_platform, serde_json, DebugAdapterBinary, 
    DebugTaskDefinition, Os, Result, Worktree,
};

// 3. Local modules (if any)
use crate::device::DeviceInfo;
```

### Documentation

- Use `///` for public APIs (visible in rustdoc)
- Use `//!` for module-level documentation
- Document all public functions and structs
- Include examples in doc comments when helpful
- Avoid inline comments; prefer self-documenting code

### Error Handling

1. **Return Result types**: `Result<T, String>` or `Result<T, ErrorType>`
2. **Clear, actionable error messages**:
   ```rust
   // Good
   Err("FVM is configured but 'fvm' command not found. Install FVM: dart pub global activate fvm".to_string())
   
   // Bad
   Err("Error".to_string())
   ```

3. **Use `?` operator** for error propagation:
   ```rust
   let config: serde_json::Value = serde_json::from_str(&config.config)
       .map_err(|e| format!("Failed to parse debug config: {e}"))?;
   ```

4. **Early returns** to reduce nesting:
   ```rust
   fn example(val: Option<i32>) -> Result<i32, String> {
       let val = val.ok_or("Value is required")?;
       if val < 0 {
           return Err("Value must be non-negative".to_string());
       }
       Ok(val)
   }
   ```

### Type System

- **Use strong types**: Create structs/enums for domain concepts
- **Never use `unwrap()`** in production code without justification
- **Use `expect()` only in tests** or when invariant is guaranteed
- **Implement common traits**: `Debug`, `Clone`, `Serialize` when appropriate

## Code Patterns

### Extension Pattern

```rust
impl zed::Extension for DartExtension {
    fn new() -> Self {
        Self {
            device_cache: Vec::new(),
            last_selected_device: None,
            #[cfg(feature = "fvm-support")]
            fvm_status: HashMap::new(),
        }
    }
}
```

### Feature-Flagged Code

```rust
#[cfg(feature = "fvm-support")]
{
    if crate::fvm::is_fvm_project(worktree, &mut self.fvm_status) {
        // FVM-specific logic
    }
}

#[cfg(not(feature = "fvm-support"))]
{
    // Non-FVM fallback
}
```

### Caching

Use `HashMap` for caching frequently accessed data:
```rust
struct DartExtension {
    device_cache: Vec<DeviceInfo>,
    last_selected_device: Option<String>,
}
```

Note: `fvm_status` cache has been removed.

### JSON Handling

Use `serde_json` for configuration parsing:
```rust
let config: serde_json::Value = serde_json::from_str(&config.config)
    .map_err(|e| format!("Failed to parse config: {e}"))?;
```

## Module Guidelines

### When to Create a New Module

- **Single responsibility**: Each module should handle one area of functionality
- **Feature flags**: Create modules for optional features (e.g., `fvm.rs`)
- **Size limit**: Consider splitting modules >400 lines
- **Clear boundaries**: Device management, debug adapter, LSP, slash commands

### Module Dependencies

```
lib.rs
  ├── device.rs (standalone, no dependencies)
  ├── debug_adapter.rs → uses device
  ├── language_server.rs (standalone)
  └── slash_commands.rs → uses device
```

Note: `fvm.rs` module has been removed.

## Extension-Specific Guidelines

### Debug Adapter

- Handle both Dart and Flutter debug scenarios
- Check user configuration in `.zed/settings.json` first
- Fall back to system dart/flutter from PATH
- Provide meaningful error messages for missing SDKs with setup instructions
- Auto-detect and cache device information
- Maintain backward compatibility with `useFvm` debug config flag

### Slash Commands

- Return guidance rather than executing directly (API constraint)
- Provide clear usage instructions
- Support argument completion where applicable

### Configuration Handling

- All SDK paths configured via `.zed/settings.json`
- No automatic FVM detection (explicit configuration required)
- Extension falls back to system dart/flutter from PATH
- Clear error messages with setup instructions when SDK not found
- See `docs/SETUP.md` for user-facing documentation

### Manual Configuration

Users must create `.zed/settings.json` in their project:

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
    "dart": { ... },
    "flutter": { ... }
  }
}
```

- Configuration is explicit and predictable
- Works with ANY directory structure
- Users have full control over SDK versions
- See `docs/SETUP.md` for comprehensive setup guide

## Pre-Commit Checklist

1. ✅ `cargo check` passes
2. ✅ `cargo test` passes (if tests exist)
3. ✅ `cargo fmt -- --check` passes
4. ✅ `cargo clippy --all-targets --all-features -- -D warnings` passes
5. ✅ Public APIs have documentation comments
6. ✅ Error messages are clear and actionable

## Dependencies

- `zed_extension_api` (0.7.0): Main Zed extension API
- `serde_json`: JSON serialization (via zed_extension_api)

**Keep dependencies minimal**. This extension aims to remain lightweight.

## Resources

- [Zed Extension Development Guide](https://zed.dev/docs/extensions/developing-extensions)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Dart LSP Specification](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)
- GitHub Issues: https://github.com/samirakhmedov/zed-ide-flutter-extension/issues
