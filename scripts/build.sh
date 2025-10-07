#!/bin/bash

# Wallmgr Build Script
# Builds all components required for Wallmgr

set -e

echo "🔨 Building Wallmgr..."

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

# Check dependencies
check_dependencies() {
    print_status "Checking dependencies..."

    local deps=("cargo" "rustc")
    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            print_error "$dep is not installed. Please install Rust first."
            exit 1
        fi
    done

    print_success "Dependencies OK"
}

# Build backend
build_backend() {
    print_status "Building backend components..."

    if [ ! -d "backend" ]; then
        print_error "backend directory not found"
        exit 1
    fi

    cd backend

    # Build all workspace members
    local members=("core" "adapters" "renderers" "connectors" "api" "daemon")

    for member in "${members[@]}"; do
        print_status "Building $member..."
        if [ -f "$member/Cargo.toml" ]; then
            cd "$member"
            cargo build --release
            cd ..
        else
            print_warning "$member directory not found"
        fi
    done

    cd ..
    print_success "Backend built successfully"
}

# Build CLI
build_cli() {
    print_status "Building CLI..."

    if [ -d "cli" ]; then
        cd cli
        cargo build --release
        cd ..
        print_success "CLI built successfully"
    else
        print_error "cli directory not found"
        exit 1
    fi
}

# Build frontend (if Qt is available)
build_frontend() {
    print_status "Checking for Qt6..."

    if ! command -v qmake6 &> /dev/null; then
        print_warning "qmake6 not found. Skipping frontend build."
        print_warning "Install Qt6 development packages to build frontend."
        return
    fi

    if [ ! -d "frontend" ]; then
        print_error "frontend directory not found"
        exit 1
    fi

    print_status "Building Qt6 QML frontend..."

    cd frontend
    mkdir -p build
    cd build

    qmake6 ..
    make -j$(nproc)

    cd ../..
    print_success "Frontend built successfully"
}

# Check for optional dependencies
check_optional() {
    print_status "Checking optional dependencies..."

    local optional_deps=(
        "mpv:video rendering"
        "linux-wallpaperengine:Wallpaper Engine support"
        "qmake6:Qt6 frontend"
    )

    for dep_info in "${optional_deps[@]}"; do
        local dep=$(echo "$dep_info" | cut -d':' -f1)
        local description=$(echo "$dep_info" | cut -d':' -f2)

        if command -v "$dep" &> /dev/null; then
            print_success "$dep found - $description enabled"
        else
            print_warning "$dep not found - $description disabled"
        fi
    done
}

# Create distribution directories
create_dist() {
    print_status "Creating distribution..."

    mkdir -p dist/bin

    # Copy binaries
    if [ -f "target/release/wallmgr-daemon" ]; then
        cp target/release/wallmgr-daemon dist/bin/
    fi

    if [ -f "target/release/wallmgr" ]; then
        cp target/release/wallmgr dist/bin/
    fi

    if [ -f "frontend/build/wallmgr-gui" ]; then
        cp frontend/build/wallmgr-gui dist/bin/
    fi

    print_success "Distribution created in dist/"
}

# Show build summary
show_summary() {
    echo
    echo "=============================================="
    print_success "Wallmgr build complete!"
    echo
    echo "Built components:"
    echo "  ✓ Backend daemon (wallmgr-daemon)"
    echo "  ✓ CLI tool (wallmgr)"
    echo "  ✓ 5 backend crates"
    echo

    if [ -f "dist/bin/wallmgr-gui" ]; then
        echo "  ✓ GUI application (wallmgr-gui)"
    fi

    echo
    echo "Installation:"
    echo "  sudo ./scripts/install.sh"
    echo
    echo "=========================================="
}

# Main build process
main() {
    echo "=============================================="
    echo "Wallmgr Build Script"
    echo "Version: $(grep -m1 "version" Cargo.toml | cut -d'"' -f2)"
    echo "=============================================="

    check_dependencies
    check_optional

    build_backend
    build_cli
    build_frontend
    create_dist

    show_summary
}

# Run main function
main "$@"