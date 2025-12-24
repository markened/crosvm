# Gfxstream Backend Screenshot Implementation Guide

This document provides instructions for implementing the screenshot capture functionality in the gfxstream backend (markened/gfxstream repository).

## Overview

The screenshot capture feature allows clients to capture the last posted frame from the gfxstream rendering pipeline. This is useful for:
- Debugging rendering issues
- Automated testing
- Performance analysis
- UI testing and validation

## Implementation Steps

### 1. Add C API Function Declaration

**File:** `include/virtgpu-gfxstream-renderer.h` or `host/virtio-gpu-gfxstream-renderer.h`

Add the following function declaration:

```c
/**
 * Captures the last posted frame from gfxstream.
 * 
 * @param width Output: Frame width in pixels
 * @param height Output: Frame height in pixels  
 * @param format Output: Pixel format (e.g., GL_RGBA)
 * @param pixels Output: Pointer to malloc'd pixel buffer (caller must free)
 * @param size Output: Size of pixel buffer in bytes
 * @return 0 on success, negative error code on failure
 */
VG_EXPORT int stream_renderer_get_screenshot(
    uint32_t* width,
    uint32_t* height,
    uint32_t* format,
    uint8_t** pixels,
    uint64_t* size);
```

### 2. Implement the Function

**File:** Create new `host/screenshot.cpp` or add to existing implementation file

```cpp
#include "host/RendererImpl.h"
#include "host/FrameBuffer.h"
#include <stdlib.h>
#include <errno.h>

extern "C" VG_EXPORT int stream_renderer_get_screenshot(
    uint32_t* width,
    uint32_t* height,
    uint32_t* format,
    uint8_t** pixels,
    uint64_t* size) {
    
    // Validate parameters
    if (!width || !height || !format || !pixels || !size) {
        return -EINVAL;
    }

    // Get renderer instance
    auto renderer = gfxstream::host::RendererImpl::get();
    if (!renderer) {
        return -ENODEV;
    }

    // Get framebuffer
    auto fb = renderer->getFrameBuffer();
    if (!fb) {
        return -ENODEV;
    }

    // Lock for thread safety if needed
    // AutoLock lock(fb->getLock());

    // Get framebuffer dimensions
    *width = fb->getWidth();
    *height = fb->getHeight();
    *format = GL_RGBA; // or use fb->getColorBufferFormat()
    
    // Calculate buffer size (RGBA = 4 bytes per pixel)
    *size = static_cast<uint64_t>(*width) * (*height) * 4;
    
    // Allocate buffer (caller will free this)
    *pixels = static_cast<uint8_t*>(malloc(*size));
    if (!*pixels) {
        return -ENOMEM;
    }

    // Read pixels from framebuffer
    // Option 1: Use existing screenshot method if available
    bool success = fb->getScreenshot(*pixels, *width, *height);
    
    // Option 2: Read from color buffer directly
    // auto colorBuffer = fb->getColorBuffer();
    // if (colorBuffer) {
    //     colorBuffer->readPixels(0, 0, *width, *height, GL_RGBA, 
    //                             GL_UNSIGNED_BYTE, *pixels);
    //     success = true;
    // }
    
    // Option 3: Use glReadPixels directly
    // fb->bind(); // Bind the framebuffer
    // glReadPixels(0, 0, *width, *height, GL_RGBA, GL_UNSIGNED_BYTE, *pixels);
    // success = (glGetError() == GL_NO_ERROR);
    
    if (!success) {
        free(*pixels);
        *pixels = nullptr;
        return -EIO;
    }

    return 0;
}
```

### 3. Add to Build System

**File:** `host/meson.build` or `CMakeLists.txt`

Add the screenshot implementation file to the build:

```python
# meson.build
gfxstream_backend_sources = [
    'RendererImpl.cpp',
    'FrameBuffer.cpp',
    # ... existing sources ...
    'screenshot.cpp',  # Add this
]
```

Or for CMake:

```cmake
# CMakeLists.txt
set(GFXSTREAM_BACKEND_SOURCES
    RendererImpl.cpp
    FrameBuffer.cpp
    # ... existing sources ...
    screenshot.cpp  # Add this
)
```

### 4. Export the Symbol

Ensure the function is exported in the shared library:

**File:** `gfxstream_backend.def` or similar export file

```
stream_renderer_get_screenshot
```

## Integration Points

### Using Existing FrameBuffer Methods

Gfxstream's FrameBuffer class likely already has methods for reading back pixels:

1. **Check for existing screenshot methods:**
   ```cpp
   // Look in host/FrameBuffer.h for methods like:
   bool getScreenshot(uint8_t* pixels, uint32_t width, uint32_t height);
   void readColorBuffer(uint32_t colorBuffer, ...);
   ```

2. **Use ColorBuffer readback:**
   ```cpp
   auto colorBuffer = fb->getColorBuffer();
   if (colorBuffer) {
       colorBuffer->readPixels(0, 0, width, height, 
                               GL_RGBA, GL_UNSIGNED_BYTE, pixels);
   }
   ```

3. **Direct OpenGL readback:**
   ```cpp
   fb->bind();
   glReadPixels(0, 0, width, height, GL_RGBA, GL_UNSIGNED_BYTE, pixels);
   ```

### Thread Safety Considerations

Gfxstream uses locks for thread safety. Ensure you:

1. Acquire necessary locks before accessing framebuffer
2. Use existing synchronization primitives
3. Handle the case where rendering is in progress

```cpp
// Example thread-safe implementation
AutoLock lock(fb->getLock());
// ... access framebuffer ...
```

## Testing

### Unit Tests

Add tests to gfxstream's test suite:

```cpp
TEST_F(ScreenshotTest, CaptureBasicFrame) {
    uint32_t width, height, format;
    uint8_t* pixels = nullptr;
    uint64_t size;
    
    int ret = stream_renderer_get_screenshot(&width, &height, &format, 
                                              &pixels, &size);
    EXPECT_EQ(ret, 0);
    EXPECT_NE(pixels, nullptr);
    EXPECT_GT(width, 0);
    EXPECT_GT(height, 0);
    EXPECT_EQ(size, width * height * 4);
    
    free(pixels);
}
```

### Integration Tests

Test with crosvm:

```bash
# Build gfxstream with screenshot support
cd gfxstream
meson setup build/ -Denable_screenshot=true
meson compile -C build/
meson install -C build/

# Build crosvm
cd crosvm
cargo build --features gpu,gfxstream

# Test screenshot capture
./target/debug/crosvm gpu screenshot --output test.ppm /tmp/vm.sock
```

## Performance Considerations

1. **Minimize copies:** Reuse existing framebuffer readback paths
2. **Async readback:** Consider PBO (Pixel Buffer Object) for async GPU readback
3. **Format conversion:** Avoid unnecessary format conversions
4. **Caching:** Don't cache screenshots - always read fresh data

## Error Handling

Return appropriate error codes:

- `-EINVAL` (22): Invalid parameters
- `-ENODEV` (19): Renderer/framebuffer not initialized
- `-ENOMEM` (12): Memory allocation failed
- `-EIO` (5): I/O error during pixel readback

## Debugging

Add debug logging:

```cpp
#ifdef DEBUG
    fprintf(stderr, "Screenshot capture: %ux%u, format=%u, size=%lu\n",
            *width, *height, *format, *size);
#endif
```

## References

- Gfxstream FrameBuffer implementation
- OpenGL glReadPixels documentation
- Existing screenshot/capture code in gfxstream
- VirtIO-GPU protocol specification
