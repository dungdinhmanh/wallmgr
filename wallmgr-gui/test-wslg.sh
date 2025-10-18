#!/bin/bash
# Quick test if WSLg is working

echo "=== WSLg Test ==="
echo ""

# Set env
export DISPLAY=:0
export WAYLAND_DISPLAY=wayland-0
export XDG_RUNTIME_DIR=/run/user/$(id -u)

echo "Environment:"
echo "  DISPLAY=$DISPLAY"
echo "  WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
echo "  XDG_RUNTIME_DIR=$XDG_RUNTIME_DIR"
echo ""

# Check X11 socket
if [ -S /tmp/.X11-unix/X0 ]; then
    echo "✓ X11 socket exists"
else
    echo "❌ X11 socket NOT found"
    echo "   Try: wsl --shutdown, then restart"
    exit 1
fi

# Test with xeyes if available
if command -v xeyes &> /dev/null; then
    echo ""
    echo "Testing with xeyes..."
    timeout 3 xeyes &
    echo "If xeyes window appeared, WSLg is working!"
else
    echo ""
    echo "Install xeyes to test: sudo apt install x11-apps"
fi

echo ""
echo "If no errors above, run: ./run-wsl.sh"
