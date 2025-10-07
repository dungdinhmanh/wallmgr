# 🎬 Hình nền động - Wallmgr

Wallmgr hỗ trợ **3 loại hình nền động** hoàn toàn riêng biệt:

## 📋 Tổng quan

| Type | Engine | Format | Use Case |
|------|--------|--------|----------|
| **Video** | mpv | MP4, WebM, MKV | Video clips, loops |
| **Spine** | spine-runtime | .skel + .atlas + .png | 2D character animations |
| **Wallpaper Engine** | linux-wallpaperengine | project.json + scenes | Full WE projects |

---

## 1️⃣ Video Wallpapers

### 📦 File Formats
- **MP4** - H.264/H.265 codec
- **WebM** - VP8/VP9 codec
- **MKV** - Matroska container
- **AVI** - Legacy support
- **GIF** - Animated GIF (looping support)

### 🔧 Rendering Engine
**mpv** - Professional-grade media player

### ⚙️ Features

#### Hardware Acceleration
```bash
--hwdec=auto          # Auto-detect GPU (VAAPI, VDPAU, NVDEC)
--vo=gpu              # GPU-based video output
```

#### Performance Settings
```bash
--fps=30              # Limit to 30fps
--profile=low-latency # Reduce latency
--loop                # Infinite loop
--no-audio            # Disable audio
```

#### Display Settings
```bash
--fs                  # Fullscreen mode
--no-border           # Borderless window
--ontop=no            # Keep behind other windows
--no-osc              # No on-screen controls
```

### 📁 File Structure
```
/home/user/wallpapers/
├── nature-loop.mp4
├── city-night.webm
└── anime-reaction.gif
```

### 💻 Usage
```bash
# Add video/GIF wallpaper
wallmgr add ~/wallpapers/nature-loop.mp4
wallmgr add ~/wallpapers/anime.gif

# Set as wallpaper
wallmgr set ~/wallpapers/nature-loop.mp4
wallmgr set ~/wallpapers/anime.gif

# API
curl -X POST http://localhost:9527/api/wallpapers/set \
  -d '{"path": "/home/user/wallpapers/anime.gif"}'
```

### ✅ Pros
- ✅ Simple format
- ✅ Hardware accelerated
- ✅ Low CPU usage
- ✅ Works on X11 + Wayland
- ✅ Wide codec support

### ⚠️ Cons
- ❌ No interactivity
- ❌ Large file sizes
- ❌ Fixed animation

---

## 2️⃣ Spine Animations (Standalone)

### 📦 File Formats
Standalone Spine 2D skeletal animations - **KHÔNG liên quan đến Wallpaper Engine**
làm  
**Required files:**
- **`.skel`** - Skeleton binary (bones, slots, attachments)
- **`.atlas`** - Texture atlas metadata
- **`.png`** - Sprite texture sheets

**Optional:**
- **`.json`** - Skeleton JSON (alternative to .skel)
- Multiple `.png` files for different texture pages

**⚠️ Quan trọng:** Spine là format độc lập, KHÔNG được sử dụng trong Wallpaper Engine!

### 🔧 Rendering Engine
**spine-runtime** hoặc custom renderer

Options:
- `spine-cpp` - Official C++ runtime
- `spine-wallpaper` - Custom wallpaper renderer (cần tạo)
- `spine-player` - Generic player

### 📁 File Structure

```
/home/user/wallpapers/anime-character/
├── character.skel       # Skeleton definition
├── character.atlas      # Texture atlas
├── character.png        # Texture sheet 1
└── character2.png       # Texture sheet 2 (if needed)
```

**NO** `project.json` file (khác với Wallpaper Engine)

### 🎨 Content Types
- **Character animations** - Idle, blinking, breathing
- **2D scenes** - Parallax layers
- **UI elements** - Animated menus
- **Effects** - Particles, glows

### ⚙️ Features

#### Animation Settings
```bash
--skeleton character.skel  # Skeleton file
--dir /path/to/project     # Project directory
--fullscreen               # Fullscreen mode
--fps 30                   # Frame rate limit
--loop                     # Loop animation
```

#### Display Settings
```bash
--monitor eDP-1            # Target monitor
--wayland                  # Use Wayland backend
--x11                      # Use X11 backend
```

### 💻 Usage
```bash
# Add Spine wallpaper (point to directory or .skel file)
wallmgr add ~/wallpapers/anime-character/
wallmgr add ~/wallpapers/anime-character/character.skel

# Set as wallpaper
wallmgr set ~/wallpapers/anime-character/

# API
curl -X POST http://localhost:9527/api/wallpapers/add \
  -d '{"path": "/home/user/wallpapers/anime-character/"}'
```

### 🔍 Auto-detection
```rust
// Spine if:
// 1. Directory contains .skel file
// 2. NO project.json file (that would be WallpaperEngine)

is_spine = has_skel_file && !has_project_json
```

### ✅ Pros
- ✅ Smooth 2D animations
- ✅ Smaller file size than video
- ✅ Skeletal animation (reusable)
- ✅ Easy to modify/re-skin
- ✅ Perfect for anime/game characters

### ⚠️ Cons
- ❌ Requires Spine runtime
- ❌ Limited to 2D
- ❌ Need custom renderer
- ❌ Less common format

### 🛠️ Creating Spine Renderer

Để sử dụng Spine wallpapers, cần:

#### Option 1: Build spine-cpp runtime
```bash
git clone https://github.com/EsotericSoftware/spine-runtimes
cd spine-runtimes/spine-cpp

# Build with your graphics backend (SFML, SDL, Cocos2d, etc.)
# Example with SFML:
cd spine-sfml
mkdir build && cd build
cmake ..
make
```

#### Option 2: Create custom wallpaper renderer
```cpp
// spine-wallpaper.cpp
#include <spine/spine.h>
#include <SFML/Graphics.hpp>

// Load skeleton, create window, render loop
// Set as desktop background
// Handle monitor positioning
```

#### Option 3: Use existing spine-player
Modify existing Spine player để chạy fullscreen dưới background

---

## 3️⃣ Wallpaper Engine Projects

### 📦 File Formats
Full Wallpaper Engine projects from Steam Workshop

**Required:**
- **`project.json`** - Project metadata
- **`scene.json`** hoặc **`scene.pkg`** - Scene definition

**Commonly includes:**
- **`.gif`** - Animated GIFs (most common in WE)
- **`.mp4`, `.webm`** - Video layers
- **`.png`, `.jpg`** - Static images
- **`.glsl`** - Custom shaders
- **`.js`** - Scripts
- **`.wav`, `.mp3`** - Audio

**Note:** Wallpaper Engine wallpapers chủ yếu dùng GIF và video, KHÔNG dùng Spine (.skel/.atlas)

### 🔧 Rendering Engine
**linux-wallpaperengine** - Linux port của Wallpaper Engine

### 📁 File Structure
```
steamapps/workshop/content/431960/123456789/
├── project.json              # Required - WE project metadata
├── scene.json               # Scene definition
├── materials/               # Materials & shaders
│   └── effect.frag
├── assets/                  # Assets
│   ├── animation.gif        # Main animated content (most common)
│   ├── background.mp4       # Video background
│   ├── layer1.png           # Static layers
│   ├── layer2.png
│   └── particle.png         # Particle textures
└── scripts/                 # Optional scripts
    └── effects.js
```

### 🎨 Wallpaper Engine Scene Types

linux-wallpaperengine hỗ trợ các loại wallpaper từ **Steam Workshop**:

#### 1. Scene Wallpapers (phổ biến nhất)
- Multi-layer 2D/3D scenes
- Particle effects
- Physics simulations
- Shaders (GLSL)
- **Assets**: images, videos, audio

#### 2. Video Wallpapers
- Video files với effects overlay
- Shader post-processing
- Audio reactive

#### 3. Web Wallpapers
- HTML/CSS/JavaScript
- WebGL content
- Interactive web pages

#### 4. Application Wallpapers
- Unity exports
- Godot exports
- Native applications

**Note:** linux-wallpaperengine là **Wallpaper Engine runtime**, KHÔNG phải Spine renderer!

### ⚙️ Features

#### Project Detection
```rust
// WallpaperEngine if:
// - Directory has project.json

is_wallpaper_engine = has_file("project.json")
```

#### Command-line Args
```bash
linux-wallpaperengine \
    --dir /path/to/project \     # Project root
    --silent \                   # No console output
    --noautomute \              # Keep audio on
    --fps 30 \                  # Frame rate limit
    --screen-root eDP-1         # Target monitor
```

#### Multi-monitor
```bash
--screen-root all        # All monitors
--screen-root eDP-1      # Specific monitor
--screen-root 0          # Monitor index
```

### 💻 Usage
```bash
# Add WE project
wallmgr add ~/steamapps/workshop/content/431960/123456789/

# Set as wallpaper
wallmgr set ~/steamapps/workshop/content/431960/123456789/

# API
curl -X POST http://localhost:9527/api/wallpapers/add \
  -d '{"path": "/home/user/steamapps/workshop/123456789/"}'
```

### 🔍 Finding WE Content
```bash
# Steam Workshop location
~/.local/share/Steam/steamapps/workshop/content/431960/

# Each subdirectory is a workshop item
ls ~/.local/share/Steam/steamapps/workshop/content/431960/
# 123456789/
# 987654321/
```

### ✅ Pros
- ✅ Full WE Workshop support
- ✅ Complex multi-layer scenes
- ✅ Particle effects, physics
- ✅ Custom shaders (GLSL)
- ✅ Interactive wallpapers
- ✅ Audio support
- ✅ Huge content library

### ⚠️ Cons
- ❌ Requires linux-wallpaperengine
- ❌ Primarily X11 (limited Wayland)
- ❌ Higher resource usage
- ❌ Larger file sizes
- ❌ May need Steam Workshop

### 🛠️ Installing linux-wallpaperengine

```bash
# Install dependencies
sudo apt install build-essential cmake libglm-dev \
  libglew-dev libsdl2-dev libmpv-dev liblz4-dev \
  libzstd-dev

# Clone and build
git clone https://github.com/Almamu/linux-wallpaperengine
cd linux-wallpaperengine
cmake -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build
sudo cmake --install build
```

---

## 🔄 Renderer Selection Logic

```rust
fn select_renderer(wallpaper: &Wallpaper) -> Renderer {
    match wallpaper.wallpaper_type {
        WallpaperType::Video => {
            Renderer::Video(VideoRenderer::new())
        }

        WallpaperType::Spine => {
            Renderer::Spine(SpineRenderer::new())
        }

        WallpaperType::WallpaperEngine => {
            Renderer::WallpaperEngine(WallpaperEngineRenderer::new())
        }

        _ => panic!("Not an animated wallpaper"),
    }
}
```

---

## 📊 Comparison Table

| Feature | Video | Spine | Wallpaper Engine |
|---------|-------|-------|------------------|
| **Ease of Use** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Quality** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **File Size** | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Customization** | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Content Library** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **X11 Support** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Wayland Support** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Setup Difficulty** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ |

---

## 🎯 Use Case Recommendations

### Choose **Video** for:
- 🌊 Nature scenes (ocean, forest, rain)
- 🌃 City timelapses
- 🔥 Simple loop animations
- 📺 Movie/anime clips
- ⚡ Quick setup, low maintenance

### Choose **Spine** for:
- 👧 Anime character animations
- 🎮 Game character wallpapers
- 🎨 2D artistic animations
- 💫 Lightweight skeletal animations
- 🔧 When you want to customize/re-skin

### Choose **Wallpaper Engine** for:
- ✨ Complex multi-layer scenes
- 🎭 Interactive wallpapers
- 🌟 Particle effects + physics
- 🎵 Audio reactive wallpapers
- 📚 Access to Steam Workshop library

---

## 🛡️ Resource Usage

### Video (mpv)
```
CPU: 2-5% (with hwdec)
RAM: 100-200 MB
GPU: Minimal (video decode)
```

### Spine (spine-runtime)
```
CPU: 5-15%
RAM: 50-150 MB
GPU: Low (2D rendering)
```

### Wallpaper Engine
```
CPU: 10-30%
RAM: 200-500 MB
GPU: Medium-High (depends on scene)
```

---

## 📝 Database Schema

```sql
-- Wallpaper types
type ENUM('image', 'video', 'spine', 'wallpaper_engine')

-- Example entries
INSERT INTO wallpapers (type, path) VALUES
  ('video', '/home/user/nature.mp4'),
  ('video', '/home/user/anime.gif'),
  ('spine', '/home/user/anime-girl/'),
  ('wallpaper_engine', '/home/user/.steam/workshop/123456/');
```

---

## 🎬 Summary

Wallmgr cung cấp **3 engine riêng biệt** cho 3 loại hình nền động:

1. **Video** → mpv (MP4, WebM, GIF - simple & reliable)
2. **Spine** → spine-runtime (Skeletal 2D animations - .skel/.atlas)
3. **Wallpaper Engine** → linux-wallpaperengine (Full WE projects - mainly GIF-based)

**Key Point:**
- GIF được render qua **mpv** khi dùng standalone
- GIF trong WE projects được render qua **linux-wallpaperengine**
- Spine (.skel) KHÔNG dùng trong WE - đây là format riêng cho skeletal animation

Mỗi loại có ưu nhược điểm riêng, phù hợp với các use case khác nhau! 🚀
