# 📚 Wallmgr Documentation

## 📖 Guides

- **[Dynamic Wallpapers Guide](./FORMATS_EXPLAINED.md)** - Detailed explanation of 3 dynamic wallpaper types
- **[Wallpaper Engine vs Spine](./WALLPAPER_ENGINE_VS_SPINE.md)** - Clearing up misconceptions

## 🏗️ Architecture

### Code Architecture
- [Core Module](./architecture/core.md) - Database, configuration, types
- [Renderer System](./architecture/renderers.md) - How renderers work
- [Adapter System](./architecture/adapters.md) - Display server integration
- [API Design](./architecture/api.md) - REST + WebSocket endpoints

### File Structure
```
wallmgr/
├── backend/           # All Rust crates
│   ├── core/         # Shared types, database, config
│   └── [other crates]
├── cli/              # Command-line interface
├── scripts/          # Build, install, packaging
├── systemd/          # Service files
├── config/           # Default configurations (planned)
└── docs/            # This documentation
```

## 📡 API Reference

### REST Endpoints
- [Wallpapers API](./api/wallpapers.md)
- [Search API](./api/search.md)
- [Tags API](./api/tags.md)
- [System API](./api/system.md)

### WebSocket Events
- [Event Types](./websocket/events.md)
- [Message Format](./websocket/format.md)
- [Client Examples](./websocket/examples.md)

## 🛠️ Development

### Getting Started
1. [Development Setup](./development/setup.md)
2. [Running Tests](./development/testing.md)
3. [Contributing](development/contributing.md)

### Adding Features
- [New Renderer](./development/new_renderer.md)
- [New Adapter](./development/new_adapter.md)
- [New Booru Source](./development/new_booru.md)

## 📋 CLI Reference

### Basic Commands
- `wallmgr add <path>` - Add wallpaper to library
- `wallmgr set <id>` - Set active wallpaper
- `wallmgr list` - List all wallpapers
- `wallmgr remove <id>` - Remove wallpaper

### Advanced Commands
- `wallmgr search` - Search booru APIs
- `wallmgr daemon` - Control daemon service
- `wallmgr status` - Check system status

### Usage Examples
```bash
# Add different wallpaper types
wallmgr add ~/Pictures/wallpaper.jpg
wallmgr add ~/Videos/ocean.mp4
wallmgr add ~/Spine/character/
wallmgr add ~/.steam/workshop/123456/

# Multi-monitor support
wallmgr set <id> --monitor HDMI-1

# Search with filters
wallmgr search --tags "landscape anime" --source danbooru
```

## 🎯 Troubleshooting

### Common Issues
- [Daemon Won't Start](./troubleshooting/daemon.md)
- [Renderer Errors](./troubleshooting/renderers.md)
- [Display Detection](./troubleshooting/displays.md)
- [Dependencies](./troubleshooting/dependencies.md)

### Debug Tips
```bash
# Enable debug logging
RUST_LOG=debug wallmgr-daemon

# Check daemon status
systemctl --user status wallmgr

# Test API directly
curl http://localhost:9527/api/health
```

## 📊 Performance

### Resource Usage
- **Video/GIF**: 2-5% CPU, minimal GPU
- **Spine**: 5-15% CPU, low GPU
- **Wallpaper Engine**: 10-30% CPU, medium-high GPU

### Optimization Tips
- Use HW acceleration for videos
- Monitor FPS limits
- Check GPU drivers for WE content

## 🔐 Security

- User-level operation (no root required)
- Proper file permissions
- Systemd service hardening
- Safe process management

## 📈 Roadmap

### Completed (v1.0)
- ✅ Full backend implementation
- ✅ 3 dynamic wallpaper renderers
- ✅ REST API + WebSocket
- ✅ CLI and daemon
- ✅ Multi-platform support

### Upcoming
- 🔜 Qt6 QML frontend (optional)
- 🔜 Plugin system
- 🔜 Mobile app sync
- 🔜 Weather-based automation

## 🤝 Contributing

We welcome contributions! See [Contributing Guide](./development/contributing.md).

### Quick Ways to Help
1. **Report bugs** on GitHub issues
2. **Write documentation** for unclear areas
3. **Test on different DEs** (GNOME, KDE, etc.)
4. **Add new booru sources**

---

*This documentation is for Wallmgr v1.0.0 and may be updated frequently.*