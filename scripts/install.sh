#!/bin/bash

# Wallmgr Installation Script
# Installs Wallmgr system-wide

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if running as root (required for system-wide install)
check_root() {
    if [[ $EUID -eq 0 ]]; then
        print_error "Do not run as root. Please install as regular user or use sudo where prompted."
        exit 1
    fi
}

# Build if needed
build_if_needed() {
    if [[ ! -f "dist/bin/wallmgr-daemon" || ! -f "dist/bin/wallmgr" ]]; then
        print_status "Building Wallmgr..."
        ./scripts/build.sh
    fi

    if [[ ! -f "dist/bin/wallmgr-daemon" ]]; then
        print_error "Build failed or dist/bin/wallmgr-daemon not found"
        exit 1
    fi

    if [[ ! -f "dist/bin/wallmgr" ]]; then
        print_error "Build failed or dist/bin/wallmgr not found"
        exit 1
    fi
}

# Install binaries
install_binaries() {
    print_status "Installing binaries..."

    sudo install -Dm755 dist/bin/wallmgr-daemon /usr/bin/wallmgr-daemon
    sudo install -Dm755 dist/bin/wallmgr /usr/bin/wallmgr

    if [[ -f "dist/bin/wallmgr-gui" ]]; then
        sudo install -Dm755 dist/bin/wallmgr-gui /usr/bin/wallmgr-gui
    fi

    print_success "Binaries installed to /usr/bin/"
}

# Install configuration
install_config() {
    print_status "Installing configuration..."

    # Create default config directory
    mkdir -p ~/.config/wallmgr

    # Create default config if it doesn't exist
    if [[ ! -f ~/.config/wallmgr/config.toml ]]; then
        cat > ~/.config/wallmgr/config.toml << 'EOF'
[api]
host = "127.0.0.1"
port = 9527
max_connections = 100

[renderer]
video_fps = 30
hardware_accel = true

[sources]
enable_danbooru = true
enable_yandere = true
enable_safebooru = true
enable_gelbooru = true
EOF
        print_success "Default config created at ~/.config/wallmgr/config.toml"
    else
        print_warning "Config already exists at ~/.config/wallmgr/config.toml"
    fi
}

# Install systemd user service
install_systemd() {
    print_status "Installing systemd service..."

    mkdir -p ~/.config/systemd/user
    cp systemd/wallmgr.service ~/.config/systemd/user/

    systemctl --user daemon-reload
    systemctl --user enable wallmgr

    print_success "Systemd service installed and enabled"
    print_status "Start daemon with: systemctl --user start wallmgr"
}

# Install optional components
install_optional() {
    # Install desktop file for GUI if available
    if [[ -f "dist/bin/wallmgr-gui" ]]; then
        print_status "Installing desktop integration..."

        mkdir -p ~/.local/share/applications
        cat > ~/.local/share/applications/wallmgr.desktop << 'EOF'
[Desktop Entry]
Type=Application
Name=Wallmgr
Comment=Wallpaper Manager
Exec=wallmgr-gui
Icon=settings-desktop-wallpaper
Terminal=false
Categories=Utility;Settings;
EOF

        print_success "Desktop integration installed"
    fi

    # Install man pages if available
    if [[ -d "man" ]]; then
        print_status "Installing man pages..."
        sudo mkdir -p /usr/share/man/man1
        sudo cp man/*.1 /usr/share/man/man1/
        sudo mandb
        print_success "Man pages installed"
    fi
}

# Show post-installation instructions
show_post_install() {
    echo
    echo "=============================================="
    print_success "Wallmgr installed successfully!"
    echo
    echo "Next steps:"
    echo
    print_status "1. Start the daemon:"
    echo "   systemctl --user start wallmgr"
    echo
    print_status "2. Check status:"
    echo "   systemctl --user status wallmgr"
    echo "   wallmgr status"
    echo
    print_status "3. Add your first wallpaper:"
    echo "   wallmgr add ~/Pictures/my-wallpaper.jpg"
    echo
    print_status "4. Set wallpaper:"
    echo "   wallmgr set <wallpaper-id>"
    echo
    print_status "5. Launch GUI (if built):"
    echo "   wallmgr-gui"    echo
    echo "Configuration: ~/.config/wallmgr/config.toml"
    echo "Data directory: ~/.local/share/wallmgr"
    echo "Cache directory: ~/.cache/wallmgr"
    echo
    print_status "Optional dependencies:"
    echo "  - mpv: for video/GIF support"
    echo "  - linux-wallpaperengine: for Steam Workshop wallpapers"
    echo "  - Qt6: for GUI application"
    echo
    echo "=========================================="
}

# Uninstall function
uninstall() {
    print_warning "Uninstalling Wallmgr..."

    # Stop and disable service
    systemctl --user stop wallmgr 2>/dev/null || true
    systemctl --user disable wallmgr 2>/dev/null || true

    # Remove binaries
    sudo rm -f /usr/bin/wallmgr-daemon
    sudo rm -f /usr/bin/wallmgr
    sudo rm -f /usr/bin/wallmgr-gui

    # Remove desktop file
    rm -f ~/.local/share/applications/wallmgr.desktop

    # Remove man pages
    sudo rm -f /usr/share/man/man1/wallmgr*

    # Note: We don't remove user data and config by default
    print_warning "User data preserved. Manually remove if desired:"
    echo "  rm -rf ~/.config/wallmgr ~/.local/share/wallmgr ~/.cache/wallmgr"

    print_success "Wallmgr uninstalled"
    exit 0
}

# Main installation function
main() {
    local UNINSTALL=false

    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --uninstall)
                UNINSTALL=true
                shift
                ;;
            --help)
                echo "Usage: $0 [--uninstall] [--help]"
                echo
                echo "Options:"
                echo "  --uninstall    Uninstall Wallmgr"
                echo "  --help         Show this help"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                exit 1
                ;;
        esac
    done

    if [[ "$UNINSTALL" == true ]]; then
        uninstall
        exit 0
    fi

    echo "=============================================="
    echo "Wallmgr Installation Script"
    echo "Version: $(grep -m1 "version" Cargo.toml | cut -d'"' -f2)"
    echo "=============================================="

    check_root
    build_if_needed
    install_binaries
    install_config
    install_systemd
    install_optional

    show_post_install
}

# Run main function
main "$@"