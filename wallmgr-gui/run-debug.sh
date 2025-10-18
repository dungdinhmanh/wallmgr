#!/bin/bash
# Debug launcher - shows all errors

cd "$(dirname "$0")"

echo "=== Debug Mode ==="
echo ""

# Set environment
export DISPLAY=:0
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=/run/user/$(id -u)
export GDK_BACKEND=x11
export LIBGL_ALWAYS_INDIRECT=0
export RUST_LOG=debug
export RUST_BACKTRACE=1

# Load cargo
source ~/.cargo/env 2>/dev/null

echo "Environment:"
echo "  DISPLAY=$DISPLAY"
echo "  WAYLAND_DISPLAY=$WAYLAND_DISPLAY"  
echo "  XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR"
echo "  GDK_BACKEND=$GDK_BACKEND"
echo ""

# Check X11
if [ -S /tmp/.X11-unix/X0 ]; then
    echo "✓ X11 socket found"
else
    echo "❌ X11 socket NOT found"
fi

echo ""
echo "Running with full output..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run without filtering
cargo run --release
