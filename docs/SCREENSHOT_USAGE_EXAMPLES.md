# Screenshot Capture Usage Examples

This document demonstrates how to use the screenshot capture functionality in rutabaga_gfx and crosvm.

## Rust API Examples

### Basic Screenshot Capture

```rust
use rutabaga_gfx::{Rutabaga, RutabagaBuilder, RutabagaComponentType, write_ppm_rgba};
use std::path::Path;

fn capture_screenshot_example() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Rutabaga with gfxstream
    let capset_mask = 0x1; // gfxstream capset
    let mut rutabaga = RutabagaBuilder::new(RutabagaComponentType::Gfxstream, capset_mask)
        .build(fence_handler, None)?;
    
    // ... after rendering some frames ...
    
    // Capture screenshot
    let pixels = rutabaga.get_screenshot()?;
    
    // The pixels are in RGBA format
    // Save to PPM file (assuming 1920x1080 display)
    write_ppm_rgba(Path::new("screenshot.ppm"), 1920, 1080, &pixels)?;
    
    println!("Screenshot saved to screenshot.ppm");
    Ok(())
}
```

### Screenshot with Custom Processing

```rust
use rutabaga_gfx::{Rutabaga, Screenshot};

fn process_screenshot(rutabaga: &Rutabaga) -> Result<(), Box<dyn std::error::Error>> {
    // Capture screenshot
    let pixels = rutabaga.get_screenshot()?;
    
    // Process pixel data (e.g., analyze, compress, send over network)
    for chunk in pixels.chunks(4) {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        let a = chunk[3];
        // Process RGBA values...
    }
    
    Ok(())
}
```

### Periodic Screenshot Capture

```rust
use std::thread;
use std::time::Duration;

fn periodic_screenshots(rutabaga: &Rutabaga) {
    let mut frame_count = 0;
    
    loop {
        thread::sleep(Duration::from_secs(1));
        
        match rutabaga.get_screenshot() {
            Ok(pixels) => {
                let filename = format!("screenshot_{:04}.ppm", frame_count);
                if let Err(e) = write_ppm_rgba(
                    Path::new(&filename), 
                    1920, 
                    1080, 
                    &pixels
                ) {
                    eprintln!("Failed to save screenshot: {}", e);
                }
                frame_count += 1;
            }
            Err(e) => {
                eprintln!("Failed to capture screenshot: {}", e);
            }
        }
    }
}
```

## C API Examples

### Basic C Usage

```c
#include <rutabaga_gfx_ffi.h>
#include <stdio.h>
#include <stdlib.h>

int capture_screenshot_c(struct rutabaga* rtbg) {
    uint32_t width = 0;
    uint32_t height = 0;
    uint64_t size = 0;
    uint8_t* pixels = NULL;
    
    // Capture screenshot
    int ret = rutabaga_get_screenshot(rtbg, &width, &height, &size, &pixels);
    if (ret != 0) {
        fprintf(stderr, "Failed to capture screenshot: %d\n", ret);
        return ret;
    }
    
    printf("Screenshot captured: %ux%u, %lu bytes\n", width, height, size);
    
    // Process pixel data...
    // pixels is in RGBA format, 4 bytes per pixel
    
    // Free the buffer
    free(pixels);
    
    return 0;
}
```

### Save to PPM File in C

```c
#include <stdio.h>

int save_ppm_rgba(const char* filename, uint32_t width, uint32_t height, 
                  const uint8_t* rgba_data) {
    FILE* fp = fopen(filename, "wb");
    if (!fp) {
        return -1;
    }
    
    // Write PPM header
    fprintf(fp, "P6\n%u %u\n255\n", width, height);
    
    // Write RGB data (skip alpha channel)
    for (uint32_t i = 0; i < width * height; i++) {
        fwrite(&rgba_data[i * 4], 1, 3, fp);
    }
    
    fclose(fp);
    return 0;
}

int capture_and_save(struct rutabaga* rtbg) {
    uint32_t width, height;
    uint64_t size;
    uint8_t* pixels = NULL;
    
    int ret = rutabaga_get_screenshot(rtbg, &width, &height, &size, &pixels);
    if (ret == 0) {
        save_ppm_rgba("screenshot.ppm", width, height, pixels);
        free(pixels);
    }
    
    return ret;
}
```

## CLI Usage

### Basic Screenshot Capture

```bash
# Capture screenshot from running crosvm instance
crosvm gpu screenshot --output screenshot.ppm /run/crosvm.sock
```

### Automated Screenshot Capture

```bash
#!/bin/bash
# Capture screenshots every 5 seconds

count=0
while true; do
    output="screenshot_$(printf '%04d' $count).ppm"
    crosvm gpu screenshot --output "$output" /run/crosvm.sock
    echo "Captured $output"
    count=$((count + 1))
    sleep 5
done
```

### Convert PPM to PNG

PPM files can be converted to more common formats using ImageMagick:

```bash
# Convert single file
convert screenshot.ppm screenshot.png

# Convert all PPM files
for f in screenshot_*.ppm; do
    convert "$f" "${f%.ppm}.png"
done
```

## Integration with Testing Frameworks

### Automated Visual Testing

```rust
#[test]
fn test_rendering_output() {
    let rutabaga = setup_rutabaga_for_test();
    
    // Render test scene
    render_test_scene(&rutabaga);
    
    // Capture screenshot
    let pixels = rutabaga.get_screenshot().expect("Screenshot failed");
    
    // Compare with reference image
    let reference = load_reference_image("test_scene_reference.ppm");
    assert_pixels_match(&pixels, &reference, 0.01); // 1% tolerance
}
```

### CI/CD Integration

```yaml
# .github/workflows/visual-tests.yml
name: Visual Tests

on: [push, pull_request]

jobs:
  visual-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Build crosvm with GPU support
        run: cargo build --features gpu,gfxstream
      
      - name: Run visual tests
        run: |
          ./tests/run_visual_tests.sh
          
      - name: Upload screenshots on failure
        if: failure()
        uses: actions/upload-artifact@v2
        with:
          name: test-screenshots
          path: test_output/*.ppm
```

## Error Handling

### Rust

```rust
match rutabaga.get_screenshot() {
    Ok(pixels) => {
        // Process pixels
    }
    Err(RutabagaError::Unsupported) => {
        eprintln!("Screenshot not supported by this backend");
    }
    Err(e) => {
        eprintln!("Screenshot error: {:?}", e);
    }
}
```

### C

```c
uint32_t width, height;
uint64_t size;
uint8_t* pixels = NULL;

int ret = rutabaga_get_screenshot(rtbg, &width, &height, &size, &pixels);
switch (ret) {
    case 0:
        // Success
        process_pixels(pixels, size);
        free(pixels);
        break;
    case -EINVAL:
        fprintf(stderr, "Invalid parameters\n");
        break;
    case -ENODEV:
        fprintf(stderr, "Renderer not initialized\n");
        break;
    default:
        fprintf(stderr, "Unknown error: %d\n", ret);
        break;
}
```

## Performance Notes

1. **Synchronization**: Screenshot capture is a synchronous operation that may block rendering
2. **Memory**: Large screenshots (e.g., 4K) can use significant memory (33MB for 3840x2160 RGBA)
3. **Frequency**: Avoid capturing screenshots too frequently in production
4. **GPU readback**: The operation involves GPU-to-CPU memory transfer which can be slow

## Best Practices

1. Use screenshots for debugging and testing, not production monitoring
2. Free memory promptly after processing pixel data
3. Consider downsampling for large displays
4. Use async/background threads for processing if capturing frequently
5. Implement proper error handling
6. Document expected image dimensions and formats
