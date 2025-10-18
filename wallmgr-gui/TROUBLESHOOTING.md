# 🔧 Wallmgr GUI - Troubleshooting

## Common Issues

### Issue 1: Window doesn't appear / Crashes on startup

**Symptoms:**
```
Error: WinitEventLoop(ExitFailure(1))
Broken pipe (os error 32)
DISPLAY= (empty)
```

**Cause:** WSLg environment variables not set

**Solution 1: Use run-wsl.sh (RECOMMENDED)**
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
./run-wsl.sh
```

This script automatically sets:
- DISPLAY=:0
- WAYLAND_DISPLAY=wayland-0
- XDG_RUNTIME_DIR=/run/user/$(id -u)
- GDK_BACKEND=x11

**Solution 2: Manual export**
```bash
export DISPLAY=:0
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=/run/user/$(id -u)
export GDK_BACKEND=x11
cd /mnt/h/app/wallmgr/wallmgr-gui
cargo run --release
```

**Solution 3: Restart WSL**
```powershell
# In PowerShell (Windows side)
wsl --shutdown
# Wait 5-10 seconds
wsl
# Try again
cd /mnt/h/app/wallmgr/wallmgr-gui
./run-wsl.sh
```

**Solution 4: Test WSLg**
```bash
cd /mnt/h/app/wallmgr/wallmgr-gui
./test-wslg.sh
# Should show "✓ X11 socket exists"
```

---

### Issue 2: Clipboard errors

**Symptoms:**
```
[ERROR arboard::platform::linux::x11] Worker thread errored
Connection reset by peer
```

**Cause:** Clipboard integration with WSLg

**Solution:** Errors are non-fatal, app should still work. If clipboard is critical:
```bash
# Disable clipboard warnings
cargo run --release 2>&1 | grep -v arboard
```

---

### Issue 3: XDG Settings Portal timeout

**Symptoms:**
```
[ERROR sctk_adwaita::config] XDG Settings Portal did not return response in time
```

**Cause:** WSLg portal not responding fast enough

**Solution:** This is a warning only, doesn't affect functionality. To hide:
```bash
cargo run --release 2>&1 | grep -v "XDG Settings"
```

---

### Issue 4: Build takes forever

**Cause:** First build compiles 500+ dependencies

**Solution:**
```bash
# First build (10-15 minutes)
cargo build --release

# Subsequent builds (1-2 minutes)
cargo run --release
```

**Speed up:**
```bash
# Use debug mode for development (faster compile)
cargo run
```

---

### Issue 5: "cargo: command not found"

**Cause:** Rust not in PATH

**Solution:**
```bash
source ~/.cargo/env
cargo --version  # Should show version
```

Or:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

---

### Issue 6: Network timeout when searching

**Cause:** Booru API unreachable or rate limited

**Solution:**
1. Check internet connection
2. Try different provider
3. Wait and retry (rate limit)
4. Check if site is down: https://downdetector.com

---

### Issue 7: Images don't load / Show placeholders

**Status:** Expected behavior (TODO)

**Reason:** Thumbnail loading not yet implemented

**Current:** Shows colored boxes with resolution

**Future:** Will load actual image thumbnails

---

## Environment Check

### Check WSLg Status
```bash
echo "DISPLAY: $DISPLAY"           # Should be :0
echo "WAYLAND_DISPLAY: $WAYLAND_DISPLAY"  # Should be wayland-0 or empty
echo "XDG_RUNTIME_DIR: $XDG_RUNTIME_DIR"  # Should be /run/user/1000
```

### Test X11
```bash
# Install xeyes if needed
sudo apt install x11-apps -y

# Test X11 window
xeyes &

# If xeyes shows, X11 works!
```

### Test OpenGL
```bash
# Install glxinfo
sudo apt install mesa-utils -y

# Check OpenGL
glxinfo | grep "OpenGL version"
```

---

## Recommended Launch Command

```bash
cd /mnt/h/app/wallmgr/wallmgr-gui

# Method 1: Use script (includes all fixes)
./run-wsl.sh

# Method 2: Manual with X11
export WAYLAND_DISPLAY=""
export GDK_BACKEND=x11
export DISPLAY=:0
cargo run --release

# Method 3: Suppress warnings
./run-wsl.sh 2>&1 | grep -v -E "XDG Settings|arboard"
```

---

## Performance Tips

### Faster Compilation
```bash
# Install mold linker (optional but faster)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
cargo build --release
```

### Reduce Memory Usage
```bash
# Limit parallel jobs
cargo build --release -j 2
```

---

## Known Limitations

1. **Thumbnails:** Placeholders only (TODO: image loading)
2. **Downloads:** Context menu shows but not implemented
3. **File dialogs:** Browse buttons don't open dialogs yet
4. **Set wallpaper:** Not implemented yet
5. **Clipboard:** May show errors but copy URL works

---

## Debug Mode

### Enable Logging
```bash
RUST_LOG=debug cargo run --release
```

### Check Specific Module
```bash
RUST_LOG=wallmgr_gui=trace cargo run --release
```

---

## Platform-Specific Issues

### WSLg Not Working?

**Check Windows version:**
- Requires Windows 11 or Windows 10 21H2+
- Update WSL: `wsl --update`

**Check WSL version:**
```bash
wsl --version  # Should show WSL version 2
```

**Reinstall WSLg:**
```powershell
wsl --shutdown
wsl --update --web-download
```

---

## Getting Help

### Provide This Info

1. **Environment:**
```bash
echo "OS: $(uname -a)"
echo "Rust: $(rustc --version)"
echo "Cargo: $(cargo --version)"
echo "Display: $DISPLAY"
echo "Wayland: $WAYLAND_DISPLAY"
```

2. **Error Output:**
```bash
cargo run --release 2>&1 | tee error.log
# Share error.log
```

3. **Build Info:**
```bash
cargo --version
rustc --version
```

---

## Quick Fixes Summary

| Issue | Quick Fix |
|-------|-----------|
| Window not showing | `export GDK_BACKEND=x11` |
| Clipboard errors | Ignore (non-fatal) |
| XDG timeout | Ignore (warning only) |
| Slow build | Use `cargo run` (debug) |
| cargo not found | `source ~/.cargo/env` |

---

**Most Common:** Use `./run-wsl.sh` which includes all fixes! 🚀
