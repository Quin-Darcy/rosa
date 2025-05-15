/*
 * Copyright (C) 2025, Quin Darcy <pohmsuindraguli@gmail.com>
 *
 * This code is licensed under the MIT License.
 * See the LICENSE file in the root directory for the full license text.
 */

use std::path::PathBuf;

use crate::error::SplitterError;

/// Main struct for processing binary files
#[derive(Debug)]
#[derive(PartialEq)]
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
        file_path: PathBuf, 
        ros: usize
    ) -> Result<Self, SplitterError> {

        // Check if file exists
        if !file_path.is_file() {
            return Err(SplitterError::NotFileError);
        }

        // Attempt to get size in bits
        let bits = match file_path.metadata() {
            Ok(md) => md.len() * 8,
            Err(_) => return Err(SplitterError::MetadataError),
        };

        // Check if valid RO number given
        if ros == 0 {
            return Err(SplitterError::InvalidRoError);
        }

        // Check size is divisible by ROs
        if bits % (ros as u64) != 0 {
            return Err(SplitterError::InvalidSizeError);
        }

        let splitter = RoSplitter {
            file_path,
            size: bits,
            num_ros: ros
        };

        Ok(splitter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_splitter_new() {
        let exist_file_name = ".test_file.txt";
        let non_exist_file_name = "asdflakjs.sdf";
        let valid_ros = 20;
        let invalid_ros = 0;

        // Create the absolute paths
        let exist_file = env::current_dir().expect("failed").join(exist_file_name);
        let non_exist_file = env::current_dir().expect("failed").join(non_exist_file_name);

        println!("Exist file: {:?}", exist_file);
        println!("NOn exist file: {:?}", non_exist_file);

        assert_eq!(RoSplitter::new(non_exist_file, valid_ros), Err(SplitterError::NotFileError)); 
        assert_eq!(RoSplitter::new(exist_file.clone(), valid_ros), Err(SplitterError::InvalidSizeError));
        assert_eq!(RoSplitter::new(exist_file, invalid_ros), Err(SplitterError::InvalidRoError));
    }
}
