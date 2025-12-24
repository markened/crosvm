// Copyright 2024 The ChromiumOS Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Tests for screenshot functionality

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rutabaga_gfx::write_ppm_rgba;

    #[test]
    fn test_write_ppm_rgba() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_screenshot.ppm");

        // Create a small 2x2 RGBA test image
        let width = 2;
        let height = 2;
        let rgba_data = vec![
            255, 0, 0, 255, // Red pixel
            0, 255, 0, 255, // Green pixel
            0, 0, 255, 255, // Blue pixel
            255, 255, 255, 255, // White pixel
        ];

        let result = write_ppm_rgba(&file_path, width, height, &rgba_data);
        assert!(result.is_ok());

        // Verify file was created
        assert!(file_path.exists());

        // Read and verify file contents
        let contents = std::fs::read_to_string(&file_path).unwrap();
        assert!(contents.starts_with("P6\n"));
        assert!(contents.contains("2 2\n"));
        assert!(contents.contains("255\n"));
    }

    #[test]
    fn test_write_ppm_rgba_invalid_size() {
        use tempfile::tempdir;

        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_invalid.ppm");

        // Create data with incorrect size
        let width = 10;
        let height = 10;
        let rgba_data = vec![0; 10]; // Too small

        let result = write_ppm_rgba(&file_path, width, height, &rgba_data);
        assert!(result.is_err());
    }
}
