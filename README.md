# 🎬 Wallmgr - Advanced Linux Wallpaper Manager

> **Production-ready wallpaper manager** supporting 4 wallpaper types including Video, GIF, Spine animations, and Steam Workshop wallpapers.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-000000.svg)](https://www.rust-lang.org/)

## ✨ Unique Features

Wallmgr is the **only Linux wallpaper manager** with:

### 🎨 4 Wallpaper Types
- **Static Images**: PNG, JPG, WebP, BMP ✅
- **Video Wallpapers**: MP4, WebM, MKV, **GIF** (via mpv) ✅
- **Spine Animations**: Skeletal 2D animations (.skel + .atlas + .png) 🚧 **Coming Soon**
- **Wallpaper Engine**: Full Steam Workshop projects (via linux-wallpaperengine) ✅

### 🖥️ Universal Compatibility
- **X11**: Feh, Nitrogen, XWallpaper
- **Wayland**: Swww, Hyprpaper, Swaybg
- **Desktop Environments**: GNOME, KDE, XFCE, LXQt, Cinnamon
- **Window Managers**: Hyprland, Sway, i3, Openbox, Niri

### 🔧 Complete Toolchain
- **REST API** + WebSocket for real-time events
- **CLI Tool** for automation and scripting
- **Systemd Service** for auto-start and monitoring
- **Build Scripts** and packaging for distribution

---

## 🚀 Quick Start

### Install from Source

```bash
# Clone and build
git clone https://github.com/dungdinhmanh/wallmgr.git
cd wallmgr

# Build all components
./scripts/build.sh

# Install system-wide
sudo ./scripts/install.sh
```

### Basic Usage

```bash
# Start daemon (auto-detects your desktop environment)
systemctl --user start wallmgr

# Add wallpapers
wallmgr add ~/Pictures/wallpaper.jpg
wallmgr add ~/Videos/ocean.mp4
wallmgr add ~/Spine/character/
wallmgr add ~/.steam/workshop/123456/

# Set wallpaper
wallmgr set <wallpaper-id>
wallmgr set <wallpaper-id> --monitor eDP-1

# Search and download
wallmgr search --tags "anime 4k" --source danbooru
wallmgr list --type video
```

### REST API

```bash
# Health check
curl http://localhost:9527/api/health

# List wallpapers
curl http://localhost:9527/api/wallpapers

# Set wallpaper
curl -X POST http://localhost:9527/api/wallpapers/set \
  -d '{"wallpaper_id": "uuid", "monitor": null}'
```

---

## 🏗️ Architecture

### Backend (7 Rust Crates)
```
wallmgr/
├── core/         # Database, configuration, types, error handling
├── adapters/     # Display server integration (X11, Wayland, DEs)
├── renderers/    # 3 dynamic wallpaper renderers
├── connectors/   # Booru API clients (Danbooru, Yande.re, etc.)
├── api/          # REST API + WebSocket server
├── daemon/       # Main daemon process
└── cli/          # Command-line interface
```

### Key Components

#### Renderers (3 Types)
```rust
pub enum Renderer {
    Video(VideoRenderer),           // mpv for videos/GIFs
    Spine(SpineRenderer),           // spine-runtime for animations
    WallpaperEngine(WERenderer),    // linux-wallpaperengine for scenes
}
```

#### Database Schema
```sql
CREATE TABLE wallpapers (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,  -- 'image', 'video', 'spine', 'wallpaper_engine'
    path TEXT NOT NULL,
    hash TEXT UNIQUE,
    -- ... metadata
);

CREATE TABLE tags (id, name, category, count);
CREATE TABLE wallpaper_tags (wallpaper_id, tag_id);
```

#### Auto-Detection Logic
```rust
fn detect_type(path: &Path) -> WallpaperType {
    // Priority: WE > Spine > Video/Image
    if has_file("project.json") => WallpaperEngine
    if has_skel() && !has_project() => Spine
    else => match extension => Video/Image
}
```

---

## 📊 Feature Comparison

| Feature | Wallmgr | Variety | Komorebi | Other Managers |
|---------|---------|---------|----------|----------------|
| **Video Support** | ✅ | ✅ | ⚠️ | ⚠️ |
| **Spine Animations** | 🚧 **Coming Soon** | ❌ | ❌ | ❌ |
| **Wallpaper Engine** | ✅ **Unique** | ❌ | 🟡 | ❌ |
| **Booru Search** | ✅ | ❌ | ❌ | ❌ |
| **Multi-DE Support** | ✅ | ⚠️ | ⚠️ | ⚠️ |
| **X11 + Wayland** | ✅ | ⚠️ | ⚠️ | ⚠️ |
| **CLI + API** | ✅ | ❌ | ❌ | ❌ |
| **REST API** | ✅ | ❌ | ❌ | ❌ |
| **Packaging** | ✅ | ❌ | ❌ | ⚠️ |

---

## 🎯 Which Renderer for What?

### Video/GIF Renderer (mpv)
**Best for:**
- Nature scenes, cityscapes
- Simple loop animations
- **GIF animations** (anime reactions, memes)
- Movie/game clips

```bash
wallmgr add ~/Videos/ocean.mp4
wallmgr add ~/Pictures/anime.gif
```

### Spine Renderer (spine-runtime) 🚧 Coming Soon
**Status:** Currently under development - not yet available

**Planned features:**
- 2D character animations
- Skeletal rigging
- Smooth bone-based movement
- **Anime-style characters**

```bash
# Will be available in future release
# wallmgr add ~/Spine/character-directory/
```

### Wallpaper Engine Renderer (linux-wallpaperengine)
**Best for:**
- Steam Workshop content
- Complex multi-layer scenes
- Particle systems and effects
- **Community-created wallpapers**

```bash
wallmgr add ~/.steam/workshop/content/431960/123456/
```

---

## 🛠️ Dependencies

### Runtime (Core)
```bash
# Required
cargo rustc         # Rust compiler
mpv libmpv-dev      # Video/GIF rendering
sqlite3 libsqlite3-dev  # Database

# Optional
linux-wallpaperengine   # Wallpaper Engine support
spine-runtime           # Spine animation support
```

### GUI (Optional)
```bash
qt6-base-dev        # Qt6 development
qmake6              # QML build system
```

### Development
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Qt6 (Ubuntu/Debian)
sudo apt install qt6-base-dev qt6-declarative-dev
```

---

## 📡 API Reference

### Endpoints

#### Wallpapers
```
GET    /api/wallpapers           # List all
GET    /api/wallpapers/:id       # Get single
POST   /api/wallpapers/add       # Add new
POST   /api/wallpapers/set       # Set active
DELETE /api/wallpapers/:id       # Remove
```

#### Search
```
POST   /api/search               # Booru search
POST   /api/search/download      # Download to library
```

#### Tags
```
GET    /api/tags/autocomplete    # Tag suggestions
GET    /api/tags/search          # Search tags
```

#### System
```
GET    /api/monitors             # Monitor list
GET    /api/health               # Health check
WS     /api/ws                   # WebSocket events
```

### WebSocket Events
```javascript
// Connect to WebSocket
const ws = new WebSocket('ws://localhost:9527/api/ws');

// Listen for events
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    switch(data.type) {
        case 'wallpaper_changed':
            updateWallpaper(data.wallpaper_id);
            break;
        case 'renderer_status':
            updateStatus(data.renderer_type, data.status);
            break;
    }
};
```

---

## 🤝 Contributing

### Development Setup
```bash
# Clone repository
git clone https://github.com/dungdinhmanh/wallmgr.git
cd wallmgr

# Build components
./scripts/build.sh

# Run daemon for testing
cargo run --bin wallmgr-daemon

# Run CLI in another terminal
cargo run --bin wallmgr -- status
```

### Adding New Features

1. **Renderers**: Implement `RendererTrait` for new wallpaper types
2. **Adapters**: Add support for new DEs/WMs in `wallmgr-adapters`
3. **Connectors**: Add new booru APIs in `wallmgr-connectors`
4. **CLI Commands**: Extend clap commands in `cli/src/main.rs`

### Testing
```bash
# Run tests
cargo test

# Launch daemon and test API
cargo run --bin wallmgr-daemon &
curl http://localhost:9527/api/health

# Test CLI commands
cargo run --bin wallmgr -- list
```

---

## 📚 Documentation

### User Guides
- [Dynamic Wallpapers Guide](./FORMATS_EXPLAINED.md)
- [CLI Reference](./docs/cli.md) (planned)
- [API Documentation](./docs/api.md) (planned)

### Architecture
- [Wallpaper Engine vs Spine](./WALLPAPER_ENGINE_VS_SPINE.md)
- [Implementation Details](./docs/architecture.md) (planned)
- [Renderer Architecture](./docs/renderers.md) (planned)

---

## 🔄 Changelog

### v1.0.0 (2025-01-07)
- ✅ Complete backend implementation
- ✅ 2 dynamic wallpaper renderers (Video/GIF + Wallpaper Engine)
- 🚧 Spine renderer (coming soon)
- ✅ REST API + WebSocket support
- ✅ CLI and daemon binaries
- ✅ Multi-platform display support
- ✅ Packaging and installation scripts
- 🔜 Qt6 QML frontend (upcoming)

---

## 📝 Known Limitations

- **Qt6 Frontend**: Optional, CLI/API provides full functionality
- **Spine Animations**: 🚧 Coming soon - currently under development
- **WallpaperEngine**: Requires linux-wallpaperengine installation
- **GPU Drivers**: Some renderers require proper OpenGL drivers

---

## 📄 License

**MIT License** - See LICENSE file

**Author**: dungdinhmanh
**Repository**: https://github.com/dungdinhmanh/wallmgr

---

*Wallmgr - The complete, production-ready Linux wallpaper manager supporting more formats than any other tool.*
