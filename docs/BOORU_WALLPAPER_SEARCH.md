# Booru Wallpaper Search Guide

## Overview

Wallmgr includes powerful booru integration with automatic landscape filtering, specifically optimized for desktop wallpaper discovery.

---

## 🎯 **Supported Booru Sites**

### **Tier 1: Wallpaper-Optimized**
1. **WallHaven** - Premium wallpaper site with category support
   - Built-in landscape filter
   - Categories: general, anime, people
   - Excellent quality, huge collection
   - API Key optional (required for NSFW)

2. **Konachan** - Desktop wallpaper focused
   - High resolution images
   - Good anime/manga wallpapers
   - Strong community curation

3. **Yande.re** - All-purpose booru
   - High quality images
   - Good balance of quantity and quality
   - Similar API to Konachan

### **Tier 2: General Boorus**
4. **Danbooru** - Largest anime image board
5. **Gelbooru** - Large variety
6. **Safebooru** - SFW-only version

---

## 📐 **Landscape Filtering**

### **Automatic Filters:**

#### 1. **HD Landscape** (Default)
```rust
WallpaperSearchFilter::hd_landscape()
```
- Minimum: 1920x1080 (Full HD)
- Aspect ratio: 1.3 - 2.4
- Excludes: portrait, square, NSFW

#### 2. **QHD Landscape** (1440p)
```rust
WallpaperSearchFilter::qhd_landscape()
```
- Minimum: 2560x1440 (2K)
- Aspect ratio: 1.3 - 2.4
- Perfect for high-DPI displays

#### 3. **UHD Landscape** (4K)
```rust
WallpaperSearchFilter::uhd_landscape()
```
- Minimum: 3840x2160 (4K)
- Aspect ratio: 1.3 - 2.4
- For 4K monitors

#### 4. **Ultrawide**
```rust
WallpaperSearchFilter::ultrawide()
```
- Minimum: 2560x1080
- Aspect ratio: 2.0 - 2.5 (21:9, 32:9)
- For ultrawide monitors

---

## 💻 **CLI Usage**

### **Basic Search (Auto Landscape)**
```bash
# Search for nature wallpapers (auto filters landscape 1920x1080+)
wallmgr search --tags "nature mountains" --source wallhaven

# Search multiple boorus
wallmgr search --tags "anime cityscape" --sources "konachan,yandere"

# Specify resolution
wallmgr search --tags "sunset" --resolution 4k --source wallhaven
```

### **Advanced Filtering**
```bash
# Ultrawide wallpapers only
wallmgr search --tags "space galaxy" --filter ultrawide

# Allow NSFW (requires API key for WallHaven)
wallmgr search --tags "anime" --nsfw --source danbooru

# Custom aspect ratio
wallmgr search --tags "landscape" --aspect-min 1.5 --aspect-max 2.0
```

### **Download Results**
```bash
# Search and download top 10
wallmgr search --tags "cyberpunk city" --limit 10 --download

# Add to library automatically
wallmgr search --tags "nature 4k" --limit 5 --add-to-library

# Preview before download
wallmgr search --tags "anime wallpaper" --preview
```

---

## 🔧 **API Usage**

### **REST API Endpoints**

#### Search Wallpapers
```http
POST /api/search
Content-Type: application/json

{
  "tags": ["nature", "mountains"],
  "sources": ["wallhaven", "konachan"],
  "limit": 20,
  "rating": "safe",
  "filter": "hd_landscape"
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "images": [
      {
        "id": "12345",
        "source": "wallhaven",
        "file_url": "https://...",
        "width": 1920,
        "height": 1080,
        "tags": ["nature", "mountains", "sunset"],
        "rating": "safe",
        "score": 150
      }
    ],
    "has_more": true,
    "total": 1250
  }
}
```

#### Download and Add to Library
```http
POST /api/search/download
Content-Type: application/json

{
  "image_id": "12345",
  "source": "wallhaven",
  "tags": ["mountains", "nature"]
}
```

---

## 📊 **Filter Examples**

### **Aspect Ratio Reference**

| Aspect Ratio | Common Resolutions | Description |
|--------------|-------------------|-------------|
| **1.33** (4:3) | 1024x768, 1600x1200 | Old monitors |
| **1.6** (16:10) | 1920x1200, 2560x1600 | Productivity monitors |
| **1.77** (16:9) | 1920x1080, 3840x2160 | Standard widescreen |
| **2.33** (21:9) | 2560x1080, 3440x1440 | Ultrawide |
| **2.37** (21:9) | 5120x2160 | Super ultrawide |

### **Custom Filter**
```rust
use wallmgr_connectors::WallpaperSearchFilter;

let filter = WallpaperSearchFilter {
    min_width: 2560,
    min_height: 1080,
    aspect_ratio_min: 2.0,
    aspect_ratio_max: 2.5,
    allow_nsfw: false,
    ..Default::default()
};

// Apply filter to results
let filtered = filter.filter(images);
```

---

## 🔑 **API Keys (Optional)**

### **WallHaven API Key**
Get your API key: https://wallhaven.cc/settings/account

```bash
# Set in config
wallmgr config set wallhaven.api_key "YOUR_API_KEY"

# Or via environment variable
export WALLHAVEN_API_KEY="YOUR_API_KEY"
```

**Benefits:**
- Access NSFW/Sketchy content
- Higher rate limits
- Search filters (colors, categories)

---

## 🎨 **Search Tips**

### **Effective Tags**

#### For Wallpapers:
```
✅ Good tags:
- "landscape", "scenery", "wallpaper"
- "4k", "highres", "absurdres"
- "nature", "city", "space", "fantasy"
- "minimalist", "abstract"

❌ Avoid:
- Character-focused tags (usually portrait)
- "chibi", "emoji" (usually square)
- Very specific tags (limits results)
```

#### Popular Wallpaper Tags by Booru:

**WallHaven:**
- `nature`, `city`, `space`, `abstract`
- Use colors: `#ff0000` (red), `#0000ff` (blue)

**Konachan/Yande.re:**
- `landscape`, `scenery`, `sky`
- `rating:safe` (SFW only)
- `highres` (high resolution)

**Danbooru:**
- `wallpaper`, `landscape`
- `absurdres` (4K+)
- `no_humans` (for nature scenes)

---

## 🚀 **Performance Tips**

### **Optimize Searches**

1. **Use specific booru sources:**
   ```bash
   # Faster: Query one source
   wallmgr search --tags "nature" --source wallhaven
   
   # Slower: Query all sources
   wallmgr search --tags "nature" --sources "all"
   ```

2. **Cache tag autocomplete:**
   ```bash
   # CLI caches tag suggestions locally
   wallmgr tags cache --source konachan
   ```

3. **Batch downloads:**
   ```bash
   # Download multiple at once (parallel)
   wallmgr search --tags "space" --limit 50 --download --parallel 5
   ```

---

## 🔍 **Examples by Use Case**

### **Gaming Setup Wallpapers**
```bash
wallmgr search \
  --tags "cyberpunk neon city" \
  --resolution 4k \
  --source wallhaven \
  --limit 20
```

### **Nature Photography**
```bash
wallmgr search \
  --tags "landscape mountains sunset" \
  --sources "konachan,yandere" \
  --resolution qhd \
  --download
```

### **Anime Wallpapers (SFW)**
```bash
wallmgr search \
  --tags "anime scenery rating:safe" \
  --source konachan \
  --filter hd_landscape \
  --add-to-library
```

### **Ultrawide Monitor**
```bash
wallmgr search \
  --tags "space galaxy" \
  --filter ultrawide \
  --source wallhaven \
  --limit 10
```

### **Minimalist Desktop**
```bash
wallmgr search \
  --tags "minimalist abstract" \
  --source wallhaven \
  --aspect-min 1.7 \
  --aspect-max 1.8 \
  --download
```

---

## 📝 **Notes**

- **Landscape detection** happens automatically - you don't need to specify
- **NSFW filtering** is enabled by default (use `--nsfw` to disable)
- **API rate limits** vary by booru - wallmgr handles retries automatically
- **Tag autocomplete** works in interactive mode: `wallmgr search -i`

---

## 🐛 **Troubleshooting**

### **No results found**
```bash
# Try broader tags
wallmgr search --tags "nature"  # Instead of "nature mountains lake sunset"

# Try different source
wallmgr search --tags "anime" --source danbooru  # Instead of wallhaven
```

### **All images are portrait**
```bash
# Force landscape filter
wallmgr search --tags "your_tags" --filter hd_landscape
```

### **API rate limit exceeded**
```bash
# Wait 60 seconds or use different source
wallmgr search --tags "your_tags" --source yandere  # Instead of danbooru
```

---

## 📚 **See Also**

- [Booru API Documentation](./API.md)
- [Filter Technical Details](./FILTERS_EXPLAINED.md)
- [CLI Reference](./CLI.md)

---

**Happy wallpaper hunting!** 🎨🖼️
