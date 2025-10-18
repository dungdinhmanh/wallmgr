# 🔄 Wallmgr Refactor Plan

## ❌ Current Issues
1. Provider dropdown in Online tab (should be in Settings)
2. Rushed implementation without proper API research
3. Not following Variety's design pattern
4. Search bar not centered
5. Incomplete booru site coverage

---

## ✅ New Design (Based on Variety)

### **Settings Tab → Add "Image Sources" Section**

Like Variety's checkbox list:
```
☑ Konachan (konachan.com)
☑ yande.re
☑ Danbooru
☑ Safebooru
☐ Gelbooru
☐ Rule34
☐ Zerochan
☐ Anime-Pictures
```

User selects which sources to search from.

### **Online Sources Tab → Simplified**

Remove provider dropdown. Layout:
```
┌────────────────────────────────────────┐
│                                        │
│          🔍 [Search tags...]           │
│                                        │
│   Trending: [tag1] [tag2] [tag3]      │
│                                        │
│   Results from: Konachan, yande.re    │
│                                        │
│   [Grid of images from all enabled]   │
│                                        │
└────────────────────────────────────────┘
```

When user searches, fetch from ALL enabled sources and merge results.

---

## 📊 Booru Sites to Support

### **Moebooru Engine:**
- konachan.com - Anime wallpapers
- yande.re - Anime images
- API: /post.json?tags=...&limit=20

### **Danbooru 2.0:**
- danbooru.donmai.us - Large collection
- API: /posts.json?tags=...&limit=20

### **Gelbooru 0.2:**
- gelbooru.com - Vast library
- safebooru.org - SFW only
- rule34.xxx - Adult (optional)
- API: /index.php?page=dapi&s=post&q=index&json=1&tags=...

### **Custom APIs:**
- Zerochan - www.zerochan.net
- Anime-Pictures - api.anime-pictures.net/api/v3/posts

---

## 🔧 Implementation Steps

### **Phase 1: Research** ✅ (Current)
- [x] List all sites from imgbrd-grabber
- [ ] Read model.ts for each engine
- [ ] Document API endpoints
- [ ] Test APIs manually (curl)

### **Phase 2: Data Model**
- [ ] Create `ImageSource` struct
  - name: String
  - url: String
  - engine_type: EngineType (Moebooru, Danbooru, Gelbooru, Custom)
  - enabled: bool
  - requires_auth: bool
  - is_nsfw: bool

- [ ] Update `AppSettings`
  - Add `enabled_sources: Vec<String>`
  - Remove `default_provider`

### **Phase 3: Settings Tab**
- [ ] Add "Image Sources" section
- [ ] Checkbox list (like Variety)
- [ ] Show site URL and type
- [ ] Save enabled sources to JSON

### **Phase 4: Online Tab**
- [ ] Remove provider dropdown
- [ ] Center search bar
- [ ] Center trending tags
- [ ] Show "Results from: X, Y, Z"
- [ ] Fetch from multiple sources in parallel
- [ ] Merge and sort results

### **Phase 5: API Implementation**
- [ ] Implement each engine type properly
- [ ] Handle different JSON formats
- [ ] Error handling per source
- [ ] Rate limiting

---

## 📝 API Documentation (to research)

### **Moebooru:**
```
GET https://konachan.com/post.json
Params:
  tags=landscape+anime
  limit=20
  page=1

Response: Array of posts
{
  "id": 12345,
  "tags": "landscape anime",
  "file_url": "https://...",
  "preview_url": "https://...",
  "sample_url": "https://...",
  "width": 1920,
  "height": 1080,
  "rating": "s"
}
```

### **Danbooru 2.0:**
```
GET https://danbooru.donmai.us/posts.json
Params:
  tags=landscape anime
  limit=20
  page=1

Response: Array of posts (different format)
```

### **Gelbooru 0.2:**
```
GET https://gelbooru.com/index.php
Params:
  page=dapi
  s=post
  q=index
  json=1
  tags=landscape anime
  limit=20
  pid={page-1}

Response: {"post": [...]}
```

---

## 🎨 UI Mockup

### **Settings Tab:**
```
┌─────────────────────────────────────────┐
│ Folders                                 │
│   Download: [~/Pictures/Wallpapers] 📁 │
│   Local:    [~/Pictures] 📁             │
│                                         │
│ Image Sources (select which to search) │
│   ☑ Konachan (konachan.com)            │
│   ☑ yande.re                            │
│   ☑ Danbooru (danbooru.donmai.us)      │
│   ☑ Safebooru (safebooru.org) [SFW]    │
│   ☐ Gelbooru (gelbooru.com)            │
│   ☐ Rule34 (rule34.xxx) [NSFW]         │
│   ☐ Zerochan (zerochan.net)            │
│   ☐ Anime-Pictures                      │
│                                         │
│ Display                                 │
│   Thumbnail size: [200px] ━━━━○━━━━━   │
│   NSFW content: ☐                       │
│                                         │
│ [💾 Save Settings]                      │
└─────────────────────────────────────────┘
```

### **Online Sources Tab:**
```
┌─────────────────────────────────────────┐
│                                         │
│                                         │
│          🔍 [Search tags here...]       │
│             [Search Button]             │
│                                         │
│   Trending: [anime] [landscape] [4k]   │
│             [scenery] [wallpaper]       │
│                                         │
│   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━   │
│                                         │
│   Searching: Konachan, yande.re,        │
│              Danbooru, Safebooru        │
│                                         │
│   Found 80 images                       │
│                                         │
│   ┌────┬────┬────┬────┐                │
│   │ Im │ Im │ Im │ Im │                │
│   │ g1 │ g2 │ g3 │ g4 │                │
│   └────┴────┴────┴────┘                │
│   (Grid of merged results)              │
│                                         │
└─────────────────────────────────────────┘
```

---

## ⚠️ Key Differences from Current Code

1. **No dropdown** - Sources selected in Settings
2. **Multi-source search** - Fetch from all enabled sources
3. **Variety-style** - Checkbox list for sources
4. **Centered UI** - Search and tags centered
5. **Proper APIs** - Research each engine correctly

---

## 🔍 Next Actions

1. ✅ Cancel current build
2. ⏳ Read all model.ts files
3. ⏳ Test APIs with curl
4. ⏳ Document exact request/response formats
5. ⏳ Create proper ImageSource system
6. ⏳ Refactor Settings tab
7. ⏳ Refactor Online tab
8. ⏳ Test thoroughly

---

**Status:** 📖 Research Phase  
**Priority:** Do it RIGHT this time, not FAST
