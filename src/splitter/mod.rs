/*
 * Copyright (C) 2025, Quin Darcy <pohmsuindraguli@gmail.com>
 *
 * License: see LICENSE file in root directory
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE, ALL OF
 * WHICH ARE HEREBY DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
 * BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
 * USE OF THIS SOFTWARE, EVEN IF NOT ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

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
