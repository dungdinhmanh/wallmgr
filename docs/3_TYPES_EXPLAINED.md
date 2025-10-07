# 🎬 3 Loại Hình Nền Động - Giải thích Chi tiết

## 📋 Overview

Wallmgr hỗ trợ **3 loại hình nền động** hoàn toàn độc lập:

```
1. Video/GIF    → mpv
2. Spine        → spine-runtime
3. WE Projects  → linux-wallpaperengine
```

**Quan trọng:** Mỗi loại có renderer riêng, KHÔNG overlap!

---

## 1️⃣ Video & GIF

### 📦 Định dạng
- MP4, WebM, MKV, AVI
- **GIF** (animated)

### 🔧 Renderer
**mpv** - Professional media player

### 📁 File Structure
```
wallpapers/
├── ocean.mp4
├── city.webm
└── anime.gif
```

### ✅ Đặc điểm
- Standalone video/GIF files
- Hardware acceleration
- 30fps limiting
- Đơn giản, dễ dùng nhất
- Không cần project file

### 💻 Usage
```bash
wallmgr add ~/Videos/nature.mp4
wallmgr add ~/Downloads/anime.gif
wallmgr set ~/Videos/nature.mp4
```

---

## 2️⃣ Spine Animations

### 📦 Định dạng
- `.skel` - Skeleton data (bones, slots)
- `.atlas` - Texture atlas metadata
- `.png` - Sprite texture sheets

### 🔧 Renderer
**spine-runtime** - Skeletal animation player

### 📁 File Structure
```
character-animation/
├── character.skel     # Required
├── character.atlas    # Required
└── character.png      # Required
```

**Không có `project.json`!**

### ✅ Đặc điểm
- Skeletal 2D animations
- Bone-based rigging
- Smooth interpolation
- Perfect cho character animations
- **KHÔNG liên quan Wallpaper Engine**

### 💻 Usage
```bash
wallmgr add ~/Animations/character/
wallmgr set ~/Animations/character/
```

### 🔍 Detection
```rust
// Has .skel file AND NO project.json
is_spine = has_skel && !has_project_json
```

---

## 3️⃣ Wallpaper Engine Projects

### 📦 Định dạng
- `project.json` - WE project metadata (REQUIRED)
- `scene.pkg` or `scene.json` - Scene data
- Assets: images, videos, shaders, audio

### 🔧 Renderer
**linux-wallpaperengine** - WE runtime for Linux

### 📁 File Structure
```
workshop-123456/
├── project.json       # REQUIRED - WE marker
├── scene.pkg         # Compiled scene
└── assets/
    ├── background.png
    ├── video.mp4
    ├── effect.frag   # GLSL shader
    └── audio.mp3
```

**KHÔNG có `.skel` hay `.atlas`!**

### ✅ Wallpaper Engine Scene Types

#### Scene Wallpapers (phổ biến nhất)
- Multi-layer 2D/3D compositions
- Particle systems
- Physics simulations
- GLSL shaders

#### Video Wallpapers
- Video với shader effects
- Audio reactive

#### Web Wallpapers
- HTML/CSS/JavaScript
- WebGL content

#### Application Wallpapers
- Unity exports
- Godot exports

### 💻 Usage
```bash
# From Steam Workshop
wallmgr add ~/.steam/workshop/content/431960/123456/
wallmgr set ~/.steam/workshop/content/431960/123456/
```

### 🔍 Detection
```rust
// Has project.json
is_we = has_file("project.json")
```

---

## 🔄 Auto-Detection Logic

```rust
fn detect_wallpaper_type(path: &Path) -> WallpaperType {
    // 1. Check for Wallpaper Engine first
    if path.join("project.json").exists() {
        return WallpaperType::WallpaperEngine;
    }

    // 2. Check for Spine (must NOT have project.json)
    if has_skel_file(path) && !path.join("project.json").exists() {
        return WallpaperType::Spine;
    }

    // 3. Check file extension
    match path.extension() {
        Some("mp4") | Some("webm") | Some("gif") => WallpaperType::Video,
        Some("skel") | Some("atlas") => WallpaperType::Spine,
        _ => WallpaperType::Unknown,
    }
}
```

**Thứ tự quan trọng:**
1. `project.json` → WallpaperEngine (ưu tiên cao nhất)
2. `.skel` + NO `project.json` → Spine
3. Extension → Video/Image

---

## ❌ Common Mistakes

### Mistake 1: "WE dùng Spine"
**Sự thật:**
- Wallpaper Engine KHÔNG dùng Spine
- WE và Spine là 2 sản phẩm hoàn toàn độc lập
- linux-wallpaperengine ≠ Spine renderer

### Mistake 2: "GIF chỉ có trong WE"
**Sự thật:**
- GIF có thể standalone (type: Video)
- GIF cũng có thể trong WE projects (type: WallpaperEngine)
- 2 cách sử dụng khác nhau

### Mistake 3: ".skel trong WE projects"
**Sự thật:**
- WE projects KHÔNG chứa .skel files
- .skel là exclusive cho Spine animations
- Nếu có .skel → đó là Spine, KHÔNG phải WE

---

## 📊 Comparison Table

| Feature | Video/GIF | Spine | Wallpaper Engine |
|---------|-----------|-------|------------------|
| **Required Files** | .mp4/.gif | .skel + .atlas | project.json |
| **Renderer** | mpv | spine-runtime | linux-wallpaperengine |
| **Source** | Any | Spine Software | Steam Workshop |
| **Complexity** | Simple | Medium | Complex |
| **GPU Usage** | Low | Low-Med | Med-High |
| **RAM Usage** | 100-200 MB | 50-150 MB | 200-500 MB |
| **Setup** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Content** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## 🎯 When to Use What?

### Use Video/GIF for:
✅ Nature loops (ocean, rain, clouds)
✅ City timelapses
✅ Anime reaction GIFs
✅ Movie/game clips
✅ Simple animations
✅ Quick setup

### Use Spine for:
✅ Character skeletal animations
✅ Anime girls with smooth movement
✅ Game character sprites
✅ 2D rigged animations
✅ Custom character wallpapers
✅ When you have .skel files

### Use Wallpaper Engine for:
✅ Steam Workshop wallpapers
✅ Complex multi-layer scenes
✅ Interactive wallpapers
✅ Shader-based effects
✅ Web/HTML wallpapers
✅ Unity/Godot exports
✅ Huge content library

---

## 💾 Database Schema

```sql
CREATE TABLE wallpapers (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,  -- 'video', 'spine', 'wallpaper_engine'
    path TEXT NOT NULL,
    ...
);

-- Examples
INSERT INTO wallpapers (type, path) VALUES
  ('video', '/home/user/ocean.mp4'),
  ('video', '/home/user/anime.gif'),
  ('spine', '/home/user/character/'),
  ('wallpaper_engine', '/home/user/.steam/workshop/123456/');
```

---

## 🛠️ Implementation

### Video Renderer
```rust
pub struct VideoRenderer {
    process: Option<Child>, // mpv process
}

// mpv --loop --hwdec=auto --fps=30 video.mp4
```

### Spine Renderer
```rust
pub struct SpineRenderer {
    process: Option<Child>, // spine-runtime process
}

// spine-wallpaper --skeleton char.skel --fullscreen
```

### WE Renderer
```rust
pub struct WallpaperEngineRenderer {
    process: Option<Child>, // linux-wallpaperengine process
}

// linux-wallpaperengine --dir /path/to/project --fps 30
```

---

## 📚 Resources

### Video/GIF
- mpv: https://mpv.io/
- Documentation: https://mpv.io/manual/

### Spine
- Website: http://esotericsoftware.com/
- Runtimes: https://github.com/EsotericSoftware/spine-runtimes
- Docs: http://esotericsoftware.com/spine-user-guide

### Wallpaper Engine
- Steam: https://store.steampowered.com/app/431960/
- Linux Port: https://github.com/Almamu/linux-wallpaperengine
- Workshop: https://steamcommunity.com/app/431960/workshop/

---

## 🎬 Summary

### Key Points

1. **3 loại HOÀN TOÀN độc lập:**
   - Video/GIF (mpv)
   - Spine (spine-runtime)
   - Wallpaper Engine (linux-wallpaperengine)

2. **Wallpaper Engine ≠ Spine:**
   - WE = Steam Workshop wallpapers
   - Spine = Skeletal animations
   - KHÔNG liên quan nhau

3. **Detection markers:**
   - WE: `project.json` (always)
   - Spine: `.skel` + NO `project.json`
   - Video: Extension only

4. **GIF flexibility:**
   - Standalone → Video type
   - In WE → WallpaperEngine type

5. **No overlap:**
   - Mỗi type có renderer riêng
   - Không thể nhầm lẫn
   - Clear separation

---

**Kết luận:** Wallmgr hỗ trợ 3 loại hình nền động với 3 renderer khác nhau. Mỗi loại phù hợp với use case riêng. KHÔNG có overlap giữa WE và Spine!
