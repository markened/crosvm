// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Screenshot utilities for saving frame buffer data.

use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::rutabaga_utils::RutabagaError;
use crate::rutabaga_utils::RutabagaResult;

/// Writes RGBA pixel data to a PPM file (RGB format).
/// PPM is a simple, portable format that doesn't require external libraries.
///
/// # Arguments
/// * `path` - Output file path
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `rgba_data` - RGBA pixel data (4 bytes per pixel)
pub fn write_ppm_rgba(
    path: &Path,
    width: u32,
    height: u32,
    rgba_data: &[u8],
) -> RutabagaResult<()> {
    let expected_size = (width * height * 4) as usize;
    if rgba_data.len() < expected_size {
        return Err(RutabagaError::InvalidIovec);
    }

    let mut file = File::create(path).map_err(|e| RutabagaError::IoError(e))?;

    // Write PPM header (P6 = binary RGB)
    writeln!(file, "P6").map_err(|e| RutabagaError::IoError(e))?;
    writeln!(file, "{} {}", width, height).map_err(|e| RutabagaError::IoError(e))?;
    writeln!(file, "255").map_err(|e| RutabagaError::IoError(e))?;

    // Write RGB data (skip alpha channel)
    for chunk in rgba_data.chunks_exact(4) {
        file.write_all(&chunk[0..3])
            .map_err(|e| RutabagaError::IoError(e))?;
    }

    Ok(())
}

/// Writes RGB pixel data to a PPM file.
///
/// # Arguments
/// * `path` - Output file path
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `rgb_data` - RGB pixel data (3 bytes per pixel)
pub fn write_ppm_rgb(
    path: &Path,
    width: u32,
    height: u32,
    rgb_data: &[u8],
) -> RutabagaResult<()> {
    let expected_size = (width * height * 3) as usize;
    if rgb_data.len() < expected_size {
        return Err(RutabagaError::InvalidIovec);
    }

    let mut file = File::create(path).map_err(|e| RutabagaError::IoError(e))?;

    // Write PPM header
    writeln!(file, "P6").map_err(|e| RutabagaError::IoError(e))?;
    writeln!(file, "{} {}", width, height).map_err(|e| RutabagaError::IoError(e))?;
    writeln!(file, "255").map_err(|e| RutabagaError::IoError(e))?;

    // Write RGB data
    file.write_all(rgb_data)
        .map_err(|e| RutabagaError::IoError(e))?;

    Ok(())
}
