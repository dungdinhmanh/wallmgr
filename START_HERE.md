# 🚀 Wallmgr - Start Here

## Quick Launch (GUI)

```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
./run-wsl.sh
```

**First time?** Takes 10-15 minutes to compile, then instant startup.

## Documentation Guide

| File | Purpose | Read When |
|------|---------|-----------|
| `wallmgr-gui/QUICKSTART.md` | Quick start (1 min) | First use ⭐ |
| `wallmgr-gui/README.md` | Full GUI guide | Learn features |
| `wallmgr-gui/DEVELOPMENT.md` | Developer guide | Contributing |
| `wallmgr-gui/TROUBLESHOOTING.md` | Fix issues | Problems |
| `FINAL_STATUS.md` | Complete status + all fixes | Summary |
| `README.md` | CLI/daemon docs | Backend use |
| `docs/BOORU_WALLPAPER_SEARCH.md` | Online sources | API details |

## Project Structure

```
wallmgr/
├── wallmgr-gui/        ← GUI application (main)
├── backend/            ← Core Rust code
├── cli/                ← CLI tool
├── docs/               ← Documentation
└── README.md           ← Project overview
```

## Features

✅ 4-tab interface  
✅ Local wallpaper browser  
✅ Online sources (5 booru providers)  
✅ Persistent settings  
✅ WSLg compatible  

## Status

- Build: ✅ Success (0 errors, 0 warnings)
- Size: ~100 MB source + 7 GB builds (run `cargo clean` to reduce)
- Docs: 13 markdown files (essential only)
- Scripts: 3 launchers (run-wsl.sh, run-debug.sh, test-wslg.sh)
- Code: ~1,100 lines (GUI)

## Quick Tips

- **Save space:** `cargo clean` in root and wallmgr-gui/
- **Rebuild:** Takes 10-15 minutes first time
- **Stuck?** Read `wallmgr-gui/TROUBLESHOOTING.md`

---

**First time? Read:** `wallmgr-gui/QUICKSTART.md` ⭐
