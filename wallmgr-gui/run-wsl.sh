#!/bin/bash
# WSLg launcher with environment fixes

cd "$(dirname "$0")"

echo "🎨 Wallmgr GUI - Starting..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Load Rust environment
if ! command -v cargo &> /dev/null; then
    echo "⚠️  Loading Rust environment..."
    source ~/.cargo/env 2>/dev/null
fi

# CRITICAL: Set WSLg environment variables
export DISPLAY=:0
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=${XDG_RUNTIME_DIR:-/run/user/$(id -u)}
export GDK_BACKEND=x11
export LIBGL_ALWAYS_INDIRECT=0

# Check X11 socket
if [ ! -S /tmp/.X11-unix/X0 ]; then
    echo "❌ ERROR: WSLg not running!"
    echo "   Try: wsl --shutdown, then restart WSL"
    exit 1
fi

echo "✓ DISPLAY=$DISPLAY"
echo "✓ WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
echo "✓ GDK_BACKEND=$GDK_BACKEND"
echo "✓ X11 socket found"
echo ""
echo "🚀 Launching GUI..."
echo ""

# Run with error filtering
cargo run --release 2>&1 | grep -v -E "XDG Settings|arboard|Broken pipe"

# Alternative:
# RUST_BACKTRACE=1 cargo run --release  # With backtrace
