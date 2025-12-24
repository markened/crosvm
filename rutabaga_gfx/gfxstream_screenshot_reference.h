// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// REFERENCE IMPLEMENTATION FOR GFXSTREAM BACKEND
// 
// This file provides a reference implementation guide for adding screenshot 
// capture support to the gfxstream backend library.
//
// Location: markened/gfxstream repository
// File to modify: include/virtgpu-gfxstream-renderer.h or similar

#ifndef GFXSTREAM_SCREENSHOT_REFERENCE_H
#define GFXSTREAM_SCREENSHOT_REFERENCE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * stream_renderer_get_screenshot - Capture a screenshot of the last posted frame
 *
 * This function captures the last rendered frame from the gfxstream backend and
 * returns it as RGBA pixel data. The implementation should:
 *
 * 1. Access the RendererImpl or FrameBuffer singleton to get the last frame
 * 2. Read back pixels from the framebuffer (using glReadPixels or similar)
 * 3. Allocate memory for the pixel data using malloc()
 * 4. Copy pixel data to the allocated buffer
 * 5. Set output parameters
 *
 * Thread Safety:
 * - Must be thread-safe and use appropriate locking if needed
 * - Should use existing gfxstream synchronization mechanisms
 *
 * @param width Output parameter for frame width
 * @param height Output parameter for frame height
 * @param format Output parameter for pixel format (e.g., 0x1908 for RGBA)
 * @param pixels Output parameter for pixel data pointer (malloc'd buffer)
 * @param size Output parameter for buffer size in bytes
 * @return 0 on success, negative error code on failure
 *
 * Example Implementation (pseudo-code):
 *
 * int stream_renderer_get_screenshot(uint32_t* width, uint32_t* height,
 *                                     uint32_t* format, uint8_t** pixels,
 *                                     uint64_t* size) {
 *     if (!width || !height || !format || !pixels || !size) {
 *         return -EINVAL;
 *     }
 *
 *     auto renderer = gfxstream::host::RendererImpl::get();
 *     if (!renderer) {
 *         return -ENODEV;
 *     }
 *
 *     auto fb = renderer->getFrameBuffer();
 *     if (!fb) {
 *         return -ENODEV;
 *     }
 *
 *     // Get framebuffer dimensions
 *     *width = fb->getWidth();
 *     *height = fb->getHeight();
 *     *format = GL_RGBA; // or appropriate format constant
 *
 *     // Calculate size (4 bytes per pixel for RGBA)
 *     *size = (*width) * (*height) * 4;
 *
 *     // Allocate buffer
 *     *pixels = (uint8_t*)malloc(*size);
 *     if (!*pixels) {
 *         return -ENOMEM;
 *     }
 *
 *     // Use existing screenshot/readback functionality in FrameBuffer
 *     // This might be fb->getScreenshot(), fb->readColorBuffer(), etc.
 *     bool success = fb->getScreenshot(*pixels, *width, *height);
 *     if (!success) {
 *         free(*pixels);
 *         *pixels = nullptr;
 *         return -EIO;
 *     }
 *
 *     return 0;
 * }
 */
int stream_renderer_get_screenshot(
    uint32_t* width,
    uint32_t* height,
    uint32_t* format,
    uint8_t** pixels,
    uint64_t* size);

#ifdef __cplusplus
}
#endif

#endif // GFXSTREAM_SCREENSHOT_REFERENCE_H
