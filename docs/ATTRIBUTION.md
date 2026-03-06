# Attribution & Credits

## Original Work

This extension is a significantly enhanced version of the [official Zed Dart extension](https://github.com/zed-extensions/dart), maintained by the Zed Extensions community.

### Original Authors

The foundational Dart integration code was created by:

- **Abdullah Alsigar** - [abdullah.alsigar@gmail.com](mailto:abdullah.alsigar@gmail.com)
- **Flo** - [flo80@users.noreply.github.com](https://github.com/flo80)
- **ybbond** - [hi@ybbond.id](mailto:hi@ybbond.id)
- **nielsenko** - [kasper@byolimit.com](mailto:kasper@byolimit.com)

### Source Repository

Original codebase: https://github.com/zed-extensions/dart

## What's New in This Version

This enhanced version extends the original extension with comprehensive Flutter development features:

### Major Additions

1. **FVM Integration**
   - Automatic Flutter Version Manager detection
   - Transparent command routing through FVM
   - Per-project version support

2. **Device Management**
   - Automatic device detection and caching
   - Intelligent device selection heuristics
   - Platform auto-detection

3. **Enhanced Debugging**
   - Auto-device selection for Flutter
   - Hot reload support on save
   - Comprehensive error messages with remediation

4. **Flutter Task Library**
   - 25+ Flutter command tasks
   - Package management commands
   - Multi-platform build support

5. **AI Assistant Integration**
   - 5 slash commands for Flutter workflows
   - Context-aware assistance

6. **Comprehensive Documentation**
   - User guides and tutorials
   - Configuration reference
   - Troubleshooting guide

### Code Changes

While preserving the core Dart functionality, this version adds:
- ~300 lines of new Rust code for device management
- ~100 lines for FVM integration
- ~150 lines for slash commands
- 9 new documentation files
- Enhanced debug adapter configuration

## License

This extension maintains the same license as the original: **Apache-2.0**

## Contributing

When contributing to this extension, please:

1. Acknowledge the original authors in your contributions
2. Maintain backward compatibility with core Dart features
3. Follow the existing code style and conventions
4. Document new features thoroughly

## Differences from Official Extension

### Feature Comparison

| Feature | Official Extension | This Enhanced Version |
|---------|-------------------|----------------------|
| Dart LSP | ✅ | ✅ |
| Dart Debugging | ✅ | ✅ |
| Flutter Debugging | ✅ | ✅ (enhanced) |
| FVM Support | ❌ | ✅ |
| Device Management | ❌ | ✅ |
| Auto Device Selection | ❌ | ✅ |
| Hot Reload on Save | ❌ | ✅ |
| Flutter Tasks | Basic (7) | Comprehensive (25+) |
| Slash Commands | ❌ | ✅ (5 commands) |
| Enhanced Error Messages | ❌ | ✅ |
| Comprehensive Docs | Basic | Extensive |

### Intention

This enhanced version aims to:
- Provide a complete Flutter development experience in Zed
- Bridge the gap between VS Code's Flutter extension and Zed
- Maintain compatibility with the official extension
- Contribute improvements back to the community

## Using the Official Extension

If you prefer a simpler setup or want the officially supported version, you can use the [original Dart extension](https://github.com/zed-extensions/dart) from the Zed extension registry.

## Credits

Special thanks to:
- The Zed team for creating an amazing editor
- The original Dart extension authors for the foundation
- The Flutter team for excellent development tools
- The community for feedback and contributions

## Contact

For issues, feature requests, or contributions specific to this enhanced version, please use this repository's issue tracker.

For issues with core Dart functionality present in the original extension, consider also checking the [official extension's issues](https://github.com/zed-extensions/dart/issues).
