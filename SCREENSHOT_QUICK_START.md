# Screenshot Support Quick Start Guide

This is a quick reference for using the screenshot capture functionality in crosvm.

## For Rust Developers

### Add Dependency
```toml
[dependencies]
rutabaga_gfx = { version = "0.1", features = ["gfxstream"] }
```

### Basic Usage
```rust
use rutabaga_gfx::{Rutabaga, write_ppm_rgba};
use std::path::Path;

// Capture screenshot
let pixels = rutabaga.get_screenshot()?;

// Save to file
write_ppm_rgba(Path::new("screenshot.ppm"), 1920, 1080, &pixels)?;
```

## For C Developers

### Include Header
```c
#include <rutabaga_gfx_ffi.h>
```

### Basic Usage
```c
uint32_t width, height;
uint64_t size;
uint8_t* pixels = NULL;

// Capture screenshot
int ret = rutabaga_get_screenshot(rtbg, &width, &height, &size, &pixels);
if (ret == 0) {
    // Use pixels...
    free(pixels);
}
```

## For End Users (CLI)

```bash
# Capture screenshot
crosvm gpu screenshot --output screenshot.ppm /run/crosvm.sock

# Convert to PNG
convert screenshot.ppm screenshot.png
```

## Gfxstream Backend Implementation

See `docs/GFXSTREAM_SCREENSHOT_IMPLEMENTATION.md` for detailed implementation guide.

### Quick Summary

1. **Add to header** (`virtgpu-gfxstream-renderer.h`):
   ```c
   VG_EXPORT int stream_renderer_get_screenshot(
       uint32_t* width, uint32_t* height, uint32_t* format,
       uint8_t** pixels, uint64_t* size);
   ```

2. **Implement function**:
   ```cpp
   int stream_renderer_get_screenshot(...) {
       auto fb = RendererImpl::get()->getFrameBuffer();
       // Get dimensions
       *width = fb->getWidth();
       *height = fb->getHeight();
       // Allocate buffer
       *pixels = malloc(*width * *height * 4);
       // Read pixels
       fb->getScreenshot(*pixels, *width, *height);
       return 0;
   }
   ```

3. **Build and install**:
   ```bash
   meson compile -C build/
   meson install -C build/
   ```

## Files Changed

### New Files:
- `rutabaga_gfx/src/screenshot.rs` - PPM utilities
- `rutabaga_gfx/src/screenshot_tests.rs` - Tests
- `docs/GFXSTREAM_SCREENSHOT_IMPLEMENTATION.md` - Implementation guide
- `docs/SCREENSHOT_USAGE_EXAMPLES.md` - Usage examples

### Modified Files:
- `rutabaga_gfx/src/gfxstream.rs` - FFI and Gfxstream impl
- `rutabaga_gfx/src/rutabaga_core.rs` - Trait and core impl
- `rutabaga_gfx/ffi/src/lib.rs` - C FFI binding
- `src/crosvm/cmdline.rs` - CLI command
- `src/main.rs` - Command handler

## Testing

```bash
# Run unit tests
cd rutabaga_gfx
cargo test screenshot

# Test with stub (no gfxstream backend needed)
cargo test --features gfxstream_stub screenshot
```

## Troubleshooting

### "Unsupported" Error
- Screenshot requires gfxstream backend
- Ensure gfxstream is built with screenshot support
- Check that `stream_renderer_get_screenshot` is exported

### Null Pointer Error
- Gfxstream backend not initialized
- Check renderer initialization
- Verify framebuffer is available

### CLI Command Not Found
- Build with: `cargo build --features gpu,gfxstream`
- Ensure GPU feature is enabled

## More Information

- **Full documentation**: `SCREENSHOT_IMPLEMENTATION_SUMMARY.md`
- **Usage examples**: `docs/SCREENSHOT_USAGE_EXAMPLES.md`
- **Gfxstream guide**: `docs/GFXSTREAM_SCREENSHOT_IMPLEMENTATION.md`
- **API reference**: `rutabaga_gfx/README.md`
