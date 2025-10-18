# 🚀 Wallmgr GUI - Quick Start

## Run Directly in WSL (WSLg)

### Simple Command (RECOMMENDED)
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
./run-wsl.sh
```

**This script sets all required WSLg environment variables automatically!**

### Or Manually (Advanced)
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
export DISPLAY=:0
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=/run/user/$(id -u)
export GDK_BACKEND=x11
source ~/.cargo/env
cargo run --release
```

### Test WSLg First
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
./test-wslg.sh
# Should show: ✓ X11 socket exists
```

### First Time (if not built yet)
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
cargo build --release
# Wait 5-10 minutes for first build
./target/release/wallmgr-gui
```

## Features

### Tab 1: Local Wallpapers 📁
- Browse folders
- Grid view
- Multi-select
- Right-click menu

### Tab 2: My Collection 🖼️
- Static images
- Video wallpapers
- Live2D models

### Tab 3: Online Sources 🌐
**Working Features:**
- 5 Booru providers (Konachan, yande.re, Danbooru, Gelbooru, WallHaven)
- Search by tags
- Trending suggestions
- NSFW toggle
- Right-click: Download, Favorite, Copy URL

### Tab 4: Settings ⚙
**Working Features:**
- Configure folders
- Default provider
- NSFW toggle
- Thumbnail size
- Save/Load settings

## Keyboard Shortcuts

- `Enter` - Search (when in search box)
- `Right-click` - Context menu
- `Tab` - Switch between tabs

## Troubleshooting

### Window doesn't appear
```bash
echo $DISPLAY          # Should show :0
echo $WAYLAND_DISPLAY  # Should show wayland-0
```

If not set:
```bash
wsl --shutdown
# Wait 5 seconds
wsl
```

### "cargo: command not found"
```bash
source ~/.cargo/env
```

### Build errors
```bash
cargo clean
cargo check
```

## Quick Test

1. Launch: `./run-wsl.sh`
2. Click "🌐 Online Sources" tab
3. Select "Konachan" provider
4. Enter "landscape" in search
5. Click "Search"
6. Wait for results
7. Right-click any image
8. Select "Copy URL"

## Files

```
wallmgr-gui/
├── src/              ← Source code
├── Cargo.toml        ← Dependencies
├── run-wsl.sh        ← Quick launch ⭐
└── target/           ← Build output (auto-generated)
```

## Development

```bash
# Check compilation
cargo check

# Run with debug symbols (faster compile)
cargo run

# Build release (optimized)
cargo build --release

# Clean build cache
cargo clean
```

## Configuration

Settings saved at: `~/.config/wallmgr/settings.json`

Default values:
- Download folder: `~/Pictures/Wallpapers`
- Provider: `konachan`
- NSFW: `false`
- Items per page: `20`

## Tips

1. **First search takes longer** - Loading fonts, initializing
2. **Use trending tags** - Click to add them
3. **Right-click for actions** - Don't forget context menu
4. **Save settings** - Click "💾 Save Settings" after changes
5. **NSFW content** - Toggle in Settings or Online tab

## Status

✅ Compiles successfully  
✅ Runs in WSLg  
✅ Online search works  
✅ Settings persist  
⏳ Thumbnails (placeholders)  
⏳ Downloads (TODO)  
⏳ Set wallpaper (TODO)  

---

**Quick Command:** `cd /mnt/h/app/wallmgr/wallmgr-gui && ./run-wsl.sh`
