/*
 * Copyright (C) 2025, Quin Darcy <pohmsuindraguli@gmail.com>
 *
 * This code is licensed under the MIT License.
 * See the LICENSE file in the root directory for the full license text.
 */

use std::path::PathBuf;

use crate::error::SplitterError;

/// Main struct for processing binary files
pub struct RoSplitter {
    file_path: PathBuf,
    num_ros: usize,
    size: u64,
}

impl RoSplitter {
    /// Method for creating new instance of `RoSplitter`
    ///
    /// # Errors
    ///
    /// Will return `Err` if `file` doesn't exist or invalid
    /// number of ROs are given.
    pub fn new(
        file: impl Into<PathBuf>, 
        ros: usize
    ) -> Result<Self, SplitterError> {

        // Convert given argument into PathBuf type
        let file_path = file.into();

        // Check if file exists
        if !file_path.is_file() {
            return Err(SplitterError::NotFileError);
        }

        // Attempt to get size in bits
        let bits = match file_path.metadata() {
            Ok(md) => md.len() * 8,
            Err(_) => return Err(SplitterError::MetadataError),
        };

        // Check size is divisible by ROs
        if bits % ros as u64 != 0 {
            return Err(SplitterError::InvalidSizeError);
        }

        // Check if valid RO number given
        if ros == 0 {
            return Err(SplitterError::InvalidRoError);
        }

        let spliter = RoSplitter {
            file_path,
            size: bits,
            num_ros: ros
        };

        Ok(splitter)
    }
}
