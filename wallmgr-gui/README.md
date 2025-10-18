# 🎨 Wallmgr GUI - Modern Wallpaper Manager

Full-featured wallpaper manager với 3-tab interface, inspired by Nitrogen + Variety.

## ✨ Features

### 📁 Tab 1: Local Wallpapers (Nitrogen-style)
- Browse and select from local folders
- Grid view with thumbnails
- Multi-select with checkboxes
- Context menu (Set wallpaper, Add to favorites)
- Folder navigation

### 🖼️ Tab 2: My Collection
**Sub-tabs:**
- **Static**: Image wallpapers (PNG, JPG, WebP)
- **Video**: Video wallpapers (MP4, WebM, MKV)
- **Live2D**: Live2D character models

**Features:**
- Character/wallpaper list
- Preview buttons (Live2D + Video)
- Checkbox multi-select
- Favorites management

### 🌐 Tab 3: Online Sources
**Providers:**
- Konachan (Desktop wallpapers)
- yande.re (All-rounder)
- Danbooru (Large collection)
- Gelbooru (Vast library)
- WallHaven (Premium quality)

**Features:**
- Provider selector dropdown
- Search bar (centered)
- Trending tag suggestions (clickable)
- Auto-complete (Tab to complete)
- Grid view với thumbnails
- Checkbox multi-select
- Context menu (Download, Favorite, Copy URL)
- Auto-show trending if no tags

### ⚙ Tab 4: Settings
- Download folder configuration
- Local folder selection
- Favorites folder
- Default provider
- NSFW toggle
- Items per page
- Thumbnail size slider
- Display mode (fill, fit, stretch, center, tile)
- Auto-set wallpaper option

## 🏗️ Architecture

```
wallmgr-gui/
├── src/
│   ├── main.rs              # App entry point + tab switching
│   ├── tabs/
│   │   ├── local_tab.rs     # Local wallpapers browser
│   │   ├── collection_tab.rs # My collection manager
│   │   ├── online_tab.rs    # Online sources (booru)
│   │   └── settings_tab.rs  # Settings panel
│   ├── models/
│   │   ├── wallpaper.rs     # Wallpaper data structure
│   │   ├── settings.rs      # App settings
│   │   └── provider.rs      # Booru providers
│   ├── components/
│   │   ├── image_grid.rs    # Reusable image grid
│   │   ├── context_menu.rs  # Right-click menu
│   │   └── tag_input.rs     # Tag input with autocomplete
│   └── utils/
│       ├── fs.rs            # File system helpers
│       └── download.rs      # Download utilities
└── Cargo.toml
```

## 🚀 Build & Run

### Development
```bash
# WSL
cd /mnt/h/app/wallmgr/wallmgr-gui
cargo run

# Native Linux
cd /path/to/wallmgr/wallmgr-gui
cargo run
```

### Release
```bash
cargo build --release
./target/release/wallmgr-gui
```

### Run in WSL with WSLg
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
cargo run
# Window will appear on Windows desktop via WSLg
```

## 📦 Dependencies

- **eframe/egui**: Modern immediate-mode GUI
- **reqwest**: HTTP client for API calls
- **tokio**: Async runtime
- **poll-promise**: Async state management
- **serde**: Serialization/deserialization
- **image**: Image processing
- **uuid**: Unique IDs

## 🎯 Usage

### Local Tab
1. Click "📁 Browse..." to select folder
2. Click thumbnails to select
3. Use checkboxes for multi-select
4. Right-click for context menu
5. "Set as Wallpaper" to apply

### Collection Tab
1. Switch between Static/Video/Live2D
2. Browse your saved collection
3. Preview Live2D models
4. Preview videos before applying

### Online Tab
1. Select provider from dropdown
2. Enter tags (space-separated)
3. Click trending tags to add them
4. Press Enter or "Search" button
5. Use checkboxes to multi-select
6. Right-click to download/favorite
7. Left empty = show trending

### Settings
1. Configure folders
2. Set default provider
3. Toggle NSFW content
4. Adjust UI (thumbnail size, columns)
5. Click "💾 Save Settings"

## ⌨️ Keyboard Shortcuts

- `Enter` - Search (when in search box)
- `Tab` - Auto-complete tag
- `Right-click` - Context menu
- `Checkbox` - Multi-select

## 🎨 Design Inspiration

### Nitrogen
- Simple folder browser
- Grid layout
- Quick wallpaper setting

### Variety
- Multiple sources
- Provider management
- Settings panel

### imgbrd-grabber  
- Tag-based search
- Provider support
- Download management

## 📝 Configuration

Settings stored in: `~/.config/wallmgr/settings.json`

```json
{
  "download_folder": "~/Pictures/Wallpapers",
  "local_folder": "~/Pictures",
  "favorites_folder": "~/Pictures/Favorites",
  "default_provider": "konachan",
  "allow_nsfw": false,
  "items_per_page": 20,
  "auto_set_wallpaper": false,
  "display_mode": "fill",
  "thumbnail_size": 200.0,
  "columns": 5
}
```

## 🔧 Future Enhancements

- [ ] Real thumbnail loading (image cache)
- [ ] Tag autocomplete API integration
- [ ] Batch download
- [ ] Favorites sync
- [ ] History tracking
- [ ] Filter presets (HD, QHD, UHD)
- [ ] Live2D preview renderer
- [ ] Video preview player
- [ ] Drag & drop support
- [ ] Wallpaper slideshow

## 🐛 Troubleshooting

### Window doesn't appear (WSLg)
```bash
# Check WSLg
echo $DISPLAY  # Should show :0
echo $WAYLAND_DISPLAY  # Should show wayland-0

# Restart WSL
wsl --shutdown
```

### "libGL error"
```bash
wsl sudo apt install -y libgl1-mesa-dev mesa-common-dev
```

### Slow thumbnail loading
- Increase `thumbnail_size` in settings
- Reduce `items_per_page`
- Use faster storage (SSD)

## 📊 Comparison

| Feature | Wallmgr GUI | Nitrogen | Variety |
|---------|-------------|----------|---------|
| **Local Browse** | ✅ | ✅ | ❌ |
| **Online Sources** | ✅ 5 providers | ❌ | ✅ 8+ |
| **Collection** | ✅ 3 types | ❌ | ❌ |
| **Live2D** | ✅ | ❌ | ❌ |
| **Video** | ✅ | ❌ | ✅ |
| **Tag Search** | ✅ | ❌ | ✅ |
| **Multi-select** | ✅ | ❌ | ✅ |
| **Settings** | ✅ | ✅ | ✅ |

## 📸 Screenshots

### Tab 1: Local Wallpapers
```
┌──────────────────────────────────────┐
│ 📁 Folder: ~/Pictures                │
│ [Browse] [Refresh]                   │
├──────────────────────────────────────┤
│ [x] IMG1  [ ] IMG2  [x] IMG3         │
│ [x] IMG4  [ ] IMG5  [ ] IMG6         │
│ [ ] IMG7  [ ] IMG8  [x] IMG9         │
└──────────────────────────────────────┘
```

### Tab 3: Online Sources
```
┌──────────────────────────────────────┐
│ Provider: [Konachan ▼]  [ ] NSFW    │
│                                       │
│      🔍 [landscape          ]         │
│         [Search]                      │
│                                       │
│ Trending: [anime] [nature] [4k]      │
├──────────────────────────────────────┤
│ [x] IMG  [ ] IMG  [x] IMG  [ ] IMG  │
│ 1920x    2560x    3840x    1920x    │
│ 1080     1440     2160     1080     │
└──────────────────────────────────────┘
```

## 📜 License

MIT License - See parent project

## 🤝 Contributing

1. Fork the repo
2. Create feature branch
3. Implement feature
4. Run `cargo test`
5. Run `cargo check`
6. Submit PR

## 🙏 Credits

- **Nitrogen**: UI inspiration
- **Variety**: Multi-source concept
- **end-4/dots-hyprland**: Booru API reference
- **imgbrd-grabber**: Tag management ideas

---

**Version:** 1.0.0  
**Last Updated:** 2025-01-XX
