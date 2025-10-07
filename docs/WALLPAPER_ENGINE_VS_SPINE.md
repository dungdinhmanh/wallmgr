# Wallpaper Engine vs Spine - Làm rõ sự khác biệt

## ⚠️ Quan trọng: Đây là 2 thứ HOÀN TOÀN KHÁC NHAU!

### 🎮 Wallpaper Engine
**Là gì:** Ứng dụng wallpaper từ Steam, có Workshop với hàng ngàn wallpapers

**Repo:** https://github.com/Almamu/linux-wallpaperengine
**Mục đích:** Port Wallpaper Engine sang Linux để sử dụng wallpapers từ Steam Workshop

**Hỗ trợ:**
- ✅ Scene wallpapers (2D/3D scenes)
- ✅ Video wallpapers
- ✅ Web wallpapers (HTML/CSS/JS)
- ✅ Application wallpapers (Unity, Godot)
- ✅ Particle effects, shaders
- ❌ **KHÔNG** hỗ trợ Spine animations

**File structure:**
```
workshop-item/
├── project.json          # WE project file
├── scene.pkg            # Compiled scene
└── assets/
    ├── image.png
    ├── video.mp4
    └── shader.frag
```

---

### 🦴 Spine
**Là gì:** 2D skeletal animation software từ Esoteric Software

**Website:** http://esotericsoftware.com/
**Mục đích:** Tạo skeletal animations cho games/apps

**Hỗ trợ:**
- ✅ Skeletal 2D animations
- ✅ Bone-based rigging
- ✅ Mesh deformation
- ✅ IK (Inverse Kinematics)
- ❌ **KHÔNG** liên quan đến Wallpaper Engine

**File structure:**
```
spine-animation/
├── character.skel       # Skeleton data
├── character.atlas      # Texture atlas
└── character.png        # Sprite sheet
```

---

## 🔍 So sánh

| Aspect | Wallpaper Engine | Spine |
|--------|------------------|-------|
| **Purpose** | Wallpaper app | Animation software |
| **Source** | Steam Workshop | Esoteric Software |
| **Content Type** | Scene/Video/Web | Skeletal animations |
| **File Marker** | project.json | .skel + .atlas |
| **Renderer** | linux-wallpaperengine | spine-runtime |
| **Related?** | ❌ NO | ❌ NO |

---

## ❌ Common Misconceptions

### Sai lầm 1: "Wallpaper Engine dùng Spine"
**Sự thật:**
- Wallpaper Engine và Spine là 2 sản phẩm hoàn toàn độc lập
- WE KHÔNG sử dụng Spine animations
- WE sử dụng scene-based system riêng

### Sai lầm 2: "linux-wallpaperengine render Spine"
**Sự thật:**
- linux-wallpaperengine chỉ render Wallpaper Engine projects
- KHÔNG render Spine animations
- Cần spine-runtime riêng cho Spine

### Sai lầm 3: ".skel files trong WE projects"
**Sự thật:**
- Wallpaper Engine projects KHÔNG chứa .skel files
- .skel là exclusive cho Spine
- WE dùng .pkg (compiled scenes) hoặc raw assets

---

## ✅ Trong Wallmgr

### Wallpaper Engine Support
```rust
// Type: WallpaperEngine
// Renderer: linux-wallpaperengine
// Detection: Has project.json
```

**Sử dụng:**
- Render Steam Workshop wallpapers
- Scene/Video/Web types
- Multi-layer compositions
- Shaders và effects

### Spine Support (RIÊNG BIỆT)
```rust
// Type: Spine
// Renderer: spine-runtime (custom)
// Detection: Has .skel, NO project.json
```

**Sử dụng:**
- Skeletal character animations
- Anime character wallpapers
- 2D rigged animations
- KHÔNG liên quan WE

---

## 📁 File Detection

### Wallpaper Engine Project
```bash
# Check 1: Has project.json?
if [ -f "project.json" ]; then
    TYPE="wallpaper_engine"
    RENDERER="linux-wallpaperengine"
fi
```

### Spine Animation
```bash
# Check 2: Has .skel AND NO project.json?
if [ -f "*.skel" ] && [ ! -f "project.json" ]; then
    TYPE="spine"
    RENDERER="spine-runtime"
fi
```

**Không thể nhầm lẫn vì:**
1. WE luôn có `project.json`
2. Spine KHÔNG BAO GIỜ có `project.json`
3. Hai format hoàn toàn khác nhau

---

## 🎯 Use Cases

### Khi nào dùng Wallpaper Engine?
- ✅ Muốn dùng wallpapers từ Steam Workshop
- ✅ Cần complex scenes với nhiều layers
- ✅ Muốn video + effects + shaders
- ✅ Interactive wallpapers
- ✅ Web-based wallpapers

### Khi nào dùng Spine?
- ✅ Có sẵn Spine animations (.skel files)
- ✅ Muốn character animations
- ✅ Cần skeletal rigging
- ✅ Anime/game character wallpapers
- ✅ Custom 2D animations

**KHÔNG overlap!** Hai use cases hoàn toàn khác nhau.

---

## 🛠️ Implementation

### Wallpaper Engine Renderer
```rust
pub struct WallpaperEngineRenderer {
    process: Option<Child>,
    linux_we_path: Option<String>,
}

impl WallpaperEngineRenderer {
    pub fn start(&mut self, path: &Path) -> Result<()> {
        // Must have project.json
        if !path.join("project.json").exists() {
            return Err(Error::NotWallpaperEngine);
        }

        // Spawn linux-wallpaperengine
        Command::new("linux-wallpaperengine")
            .arg("--dir").arg(path)
            .spawn()?;
    }
}
```

### Spine Renderer (KHÁC HOÀN TOÀN)
```rust
pub struct SpineRenderer {
    process: Option<Child>,
    spine_runtime_path: Option<String>,
}

impl SpineRenderer {
    pub fn start(&mut self, path: &Path) -> Result<()> {
        // Must have .skel
        let skel = find_skel_file(path)?;

        // Must NOT have project.json
        if path.join("project.json").exists() {
            return Err(Error::IsWallpaperEngine);
        }

        // Spawn spine-runtime
        Command::new("spine-wallpaper")
            .arg("--skeleton").arg(skel)
            .spawn()?;
    }
}
```

---

## 📚 Resources

### Wallpaper Engine
- Steam: https://store.steampowered.com/app/431960/Wallpaper_Engine/
- Linux Port: https://github.com/Almamu/linux-wallpaperengine
- Workshop: https://steamcommunity.com/app/431960/workshop/

### Spine
- Website: http://esotericsoftware.com/
- Runtimes: https://github.com/EsotericSoftware/spine-runtimes
- Docs: http://esotericsoftware.com/spine-user-guide

---

## 🎬 Summary

1. **Wallpaper Engine** = Steam app cho animated wallpapers
   - linux-wallpaperengine = Linux port
   - Scene/Video/Web types
   - project.json marker

2. **Spine** = 2D skeletal animation software
   - spine-runtime = Animation player
   - .skel + .atlas files
   - KHÔNG có project.json

3. **Hoàn toàn độc lập** - không overlap, không liên quan

4. **Wallmgr hỗ trợ CẢ HAI** - nhưng là 2 types riêng biệt:
   - `wallpaper_engine` type
   - `spine` type

---

**Kết luận:** linux-wallpaperengine KHÔNG phải Spine renderer. Đây là Wallpaper Engine runtime cho Linux, chỉ để chạy WE projects từ Steam Workshop.