use std::path::PathBuf;

use crate::error::SplitterError;

/// Main struct for processing binary files
pub struct RoSplitter {
    file_path: PathBuf,
    num_ros: usize
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

        // Check if valid RO number given
        if ros == 0 {
            return Err(SplitterError::InvalidRoError);
        }

        let split = RoSplitter {
            file_path,
            num_ros: ros
        };

        Ok(split)
    }
}
