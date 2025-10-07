# Wallmgr - Implementation Guide

## 📚 Overview

Wallmgr là ứng dụng quản lý hình nền cho Linux với hỗ trợ:
- **Static images** (PNG, JPG, WebP)
- **Video wallpapers** (MP4, WebM via mpv)
- **Wallpaper Engine projects** (Spine animations via linux-wallpaperengine)

## 🏗️ Architecture

### Backend (Rust)
```
wallmgr-core          → Database, Config, Types
wallmgr-adapters      → X11, Wayland, DE integrations
wallmgr-renderers     → Video & Wallpaper Engine renderers
wallmgr-connectors    → Booru API clients
wallmgr-api           → REST API + WebSocket
wallmgr-daemon        → Main daemon process
```

### Frontend (Qt6 QML)
- **Local Tab**: Manage local wallpapers
- **Stock Tab**: Curated collection
- **Search Tab**: Booru search with autocomplete
- **About Tab**: Info & credits

## 🔌 Wallpaper Engine Integration

**⚠️ Quan trọng:** Wallpaper Engine và Spine là 2 thứ HOÀN TOÀN KHÁC NHAU!
- **Wallpaper Engine** = Steam app với Workshop wallpapers
- **Spine** = 2D skeletal animation software (riêng biệt)
- linux-wallpaperengine KHÔNG render Spine!

### Wallpaper Engine Scene Types
- **Scene Wallpapers** - Multi-layer 2D/3D scenes
- **Video Wallpapers** - Video + effects
- **Web Wallpapers** - HTML/CSS/JS
- **Application Wallpapers** - Unity/Godot exports

### Supported Formats
- **Project Files**: `project.json` (required)
- **Scene Data**: `scene.pkg` or `scene.json`
- **Assets**: Images, videos, shaders, audio
- **NO Spine files** (.skel/.atlas)

### How It Works

#### 1. Detection
```rust
// backend/renderers/src/wallpaper_engine.rs
pub fn is_wallpaper_engine_project(path: &Path) -> bool {
    // Check for project.json ONLY
    path.join("project.json").exists()
}
```

#### 2. Rendering
```rust
// Start linux-wallpaperengine process
linux-wallpaperengine \
    --dir /path/to/project \
    --silent \
    --fps 30 \
    --screen-root <monitor>
```

#### 3. Process Management
- Spawn subprocess với proper args
- Monitor process health
- Auto-restart on failure
- Clean shutdown on stop

### Installation Requirements
```bash
# Install linux-wallpaperengine
git clone https://github.com/Almamu/linux-wallpaperengine
cd linux-wallpaperengine
cmake .
make
sudo make install
```

## 🎯 Usage Example

### Add Wallpaper Engine Project
```bash
# Via CLI
wallmgr add /path/to/wallpaper-engine/workshop/123456789

# Via API
curl -X POST http://localhost:9527/api/wallpapers/add \
  -H "Content-Type: application/json" \
  -d '{"path": "/path/to/project"}'
```

### Set WE Wallpaper
```bash
# CLI
wallmgr set /path/to/project

# API
curl -X POST http://localhost:9527/api/wallpapers/set \
  -H "Content-Type: application/json" \
  -d '{"wallpaper_id": "uuid", "monitor": "eDP-1"}'
```

## 📊 Database Schema

```sql
CREATE TABLE wallpapers (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL UNIQUE,
    filename TEXT NOT NULL,
    type TEXT NOT NULL,  -- 'image', 'video', 'wallpaper_engine'
    width INTEGER,
    height INTEGER,
    size INTEGER,
    hash TEXT UNIQUE,
    source TEXT,
    source_url TEXT,
    thumbnail_path TEXT,
    created_at TEXT,
    modified_at TEXT
);

CREATE TABLE tags (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE,
    category TEXT,
    count INTEGER
);

CREATE TABLE wallpaper_tags (
    wallpaper_id TEXT,
    tag_id INTEGER,
    FOREIGN KEY (wallpaper_id) REFERENCES wallpapers(id),
    FOREIGN KEY (tag_id) REFERENCES tags(id)
);
```

## 🔧 Configuration

### ~/.config/wallmgr/config.toml
```toml
[api]
host = "127.0.0.1"
port = 9527
max_connections = 100

[renderer]
video_fps = 30
hardware_accel = true
mpv_options = ["--loop", "--no-audio", "--hwdec=auto"]

[sources]
enable_danbooru = true
enable_yandere = true
enable_safebooru = true
enable_gelbooru = true
```

## 🚀 Development Roadmap

### Phase 1: Core Backend ✅
- [x] Database schema
- [x] Config management
- [x] Type system
- [x] Error handling

### Phase 2: Adapters ✅
- [x] X11 support (Feh, Nitrogen, XWallpaper)
- [x] Wayland support (Swww, Hyprpaper, Swaybg)
- [x] DE support (GNOME, KDE, XFCE)
- [x] Environment detection

### Phase 3: Renderers ✅
- [x] Video renderer (mpv)
- [x] Wallpaper Engine renderer (linux-wallpaperengine)
- [x] Process management
- [x] Monitor support

### Phase 4: API Connectors ✅
- [x] Danbooru
- [x] Yande.re
- [x] Safebooru
- [x] Gelbooru
- [x] Tag autocomplete

### Phase 5: REST API 🚧
- [ ] Handlers implementation
- [ ] WebSocket events
- [ ] File upload/download
- [ ] Error handling

### Phase 6: Daemon 📋
- [ ] Main binary
- [ ] Signal handling
- [ ] Auto-start support
- [ ] Logging

### Phase 7: CLI 📋
- [ ] Wallpaper management
- [ ] Search commands
- [ ] Daemon control

### Phase 8: Frontend 📋
- [ ] QML UI components
- [ ] API client
- [ ] Image preview
- [ ] Settings panel

### Phase 9: Packaging 📋
- [ ] Build scripts
- [ ] .deb package
- [ ] Flatpak
- [ ] AUR package

## 🛠️ Building

### Prerequisites
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Qt6
sudo apt install qt6-base-dev qt6-declarative-dev

# Dependencies
sudo apt install libsqlite3-dev mpv libmpv-dev

# Optional: linux-wallpaperengine
# See installation steps above
```

### Build Commands
```bash
# Backend
cargo build --release

# Run daemon
cargo run --bin wallmgr-daemon

# Run CLI
cargo run --bin wallmgr -- --help
```

## 📡 API Endpoints

### Wallpapers
- `GET /api/wallpapers` - List all wallpapers
- `GET /api/wallpapers/:id` - Get wallpaper details
- `POST /api/wallpapers/add` - Add new wallpaper
- `POST /api/wallpapers/set` - Set active wallpaper
- `DELETE /api/wallpapers/:id` - Delete wallpaper

### Search
- `POST /api/search` - Search booru sites
- `POST /api/search/download` - Download image
- `GET /api/tags/autocomplete?q=<prefix>` - Tag autocomplete

### System
- `GET /api/monitors` - List monitors
- `GET /api/health` - Health check
- `WS /api/ws` - WebSocket for events

## 🎨 Frontend Structure

```qml
// Main.qml
ApplicationWindow {
    TabBar {
        Tab { text: "Local" }
        Tab { text: "Stock" }
        Tab { text: "Search" }
        Tab { text: "About" }
    }

    StackLayout {
        LocalTab {}
        StockTab {}
        SearchTab {}
        AboutTab {}
    }
}
```

## 🐛 Debugging

### Enable Logging
```bash
# Backend
RUST_LOG=debug cargo run

# Check daemon logs
journalctl --user -u wallmgr -f
```

### Test API
```bash
# Health check
curl http://localhost:9527/api/health

# List wallpapers
curl http://localhost:9527/api/wallpapers
```

## 📦 Deployment

### User Service
```bash
# Enable daemon
systemctl --user enable wallmgr

# Start daemon
systemctl --user start wallmgr

# Status
systemctl --user status wallmgr
```

### Package Installation
```bash
# Debian/Ubuntu
sudo dpkg -i wallmgr_1.0.0_amd64.deb

# Flatpak
flatpak install wallmgr.flatpak
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Implement changes
4. Add tests
5. Submit PR

## 📄 License

MIT License - See LICENSE file

---

**Note**: Dự án hiện đang trong development. API có thể thay đổi.
