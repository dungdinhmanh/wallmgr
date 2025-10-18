#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Test script for Wallmgr Landscape Filter
Tests filter logic without needing full Rust compilation
"""
import sys
import io

# Fix Windows console encoding
if sys.platform == 'win32':
    sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

class BooruImage:
    def __init__(self, width, height, is_nsfw=False):
        self.width = width
        self.height = height
        self.is_nsfw = is_nsfw
        self.aspect_ratio = width / height if height > 0 else 0

class WallpaperSearchFilter:
    def __init__(self, min_width=1920, min_height=1080, 
                 aspect_ratio_min=1.3, aspect_ratio_max=2.4,
                 portrait_threshold=1.0, allow_nsfw=False):
        self.min_width = min_width
        self.min_height = min_height
        self.aspect_ratio_min = aspect_ratio_min
        self.aspect_ratio_max = aspect_ratio_max
        self.portrait_threshold = portrait_threshold
        self.allow_nsfw = allow_nsfw
    
    def matches(self, image):
        # Check dimensions
        if image.width < self.min_width or image.height < self.min_height:
            return False
        
        # Calculate aspect ratio
        aspect_ratio = image.width / image.height
        
        # Check if portrait (reject if below threshold)
        if aspect_ratio <= self.portrait_threshold:
            return False
        
        # Check aspect ratio range
        if aspect_ratio < self.aspect_ratio_min or aspect_ratio > self.aspect_ratio_max:
            return False
        
        # Check NSFW setting
        if not self.allow_nsfw and image.is_nsfw:
            return False
        
        return True

def run_tests():
    print("=" * 60)
    print("Testing Wallmgr Landscape Filter")
    print("=" * 60)
    
    # Test 1: HD Landscape Filter
    print("\n[TEST 1] HD Landscape Filter (1920x1080+)")
    filter_hd = WallpaperSearchFilter()
    
    tests = [
        ("1920x1080 landscape", BooruImage(1920, 1080, False), True),
        ("2560x1440 landscape", BooruImage(2560, 1440, False), True),
        ("1280x720 too small", BooruImage(1280, 720, False), False),
        ("1080x1920 portrait", BooruImage(1080, 1920, False), False),
        ("1920x1080 NSFW", BooruImage(1920, 1080, True), False),
        ("1920x1920 square", BooruImage(1920, 1920, False), False),
    ]
    
    passed = 0
    for name, image, expected in tests:
        result = filter_hd.matches(image)
        status = "✓ PASS" if result == expected else "✗ FAIL"
        print(f"  {status}: {name} - AR={image.aspect_ratio:.2f}, Got={result}, Expected={expected}")
        if result == expected:
            passed += 1
    
    print(f"\n  Result: {passed}/{len(tests)} tests passed")
    
    # Test 2: Aspect Ratio Tests
    print("\n[TEST 2] Aspect Ratio Tests")
    
    aspect_tests = [
        ("4:3 (1.33)", BooruImage(1920, 1440, False), True),   # Just above 1.3
        ("16:9 (1.77)", BooruImage(1920, 1080, False), True),  # Standard
        ("21:9 (2.33)", BooruImage(2560, 1080, False), True),  # Ultrawide
        ("1:1 (1.0)", BooruImage(1920, 1920, False), False),   # Square
        ("9:16 (0.56)", BooruImage(1080, 1920, False), False), # Portrait
    ]
    
    passed = 0
    for name, image, expected in aspect_tests:
        result = filter_hd.matches(image)
        status = "✓ PASS" if result == expected else "✗ FAIL"
        print(f"  {status}: {name} - AR={image.aspect_ratio:.2f}, Result={result}")
        if result == expected:
            passed += 1
    
    print(f"\n  Result: {passed}/{len(aspect_tests)} tests passed")
    
    # Test 3: Ultrawide Filter
    print("\n[TEST 3] Ultrawide Filter (21:9)")
    filter_ultrawide = WallpaperSearchFilter(
        min_width=2560, min_height=1080,
        aspect_ratio_min=2.0, aspect_ratio_max=2.5
    )
    
    ultrawide_tests = [
        ("2560x1080 (21:9)", BooruImage(2560, 1080, False), True),
        ("3440x1440 (21:9)", BooruImage(3440, 1440, False), True),
        ("1920x1080 (16:9)", BooruImage(1920, 1080, False), False),  # Too narrow
    ]
    
    passed = 0
    for name, image, expected in ultrawide_tests:
        result = filter_ultrawide.matches(image)
        status = "✓ PASS" if result == expected else "✗ FAIL"
        print(f"  {status}: {name} - AR={image.aspect_ratio:.2f}, Result={result}")
        if result == expected:
            passed += 1
    
    print(f"\n  Result: {passed}/{len(ultrawide_tests)} tests passed")
    
    # Test 4: 4K Filter
    print("\n[TEST 4] 4K Filter (3840x2160+)")
    filter_4k = WallpaperSearchFilter(min_width=3840, min_height=2160)
    
    uhd_tests = [
        ("3840x2160 (4K)", BooruImage(3840, 2160, False), True),
        ("2560x1440 (2K)", BooruImage(2560, 1440, False), False),  # Too small
    ]
    
    passed = 0
    for name, image, expected in uhd_tests:
        result = filter_4k.matches(image)
        status = "✓ PASS" if result == expected else "✗ FAIL"
        print(f"  {status}: {name} - Result={result}")
        if result == expected:
            passed += 1
    
    print(f"\n  Result: {passed}/{len(uhd_tests)} tests passed")
    
    print("\n" + "=" * 60)
    print("✅ All Filter Tests Complete!")
    print("=" * 60)

if __name__ == "__main__":
    run_tests()
