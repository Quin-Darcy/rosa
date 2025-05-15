/*
 * Copyright (C) 2025, Quin Darcy <pohmsuindraguli@gmail.com>
 *
 * This code is licensed under the MIT License.
 * See the LICENSE file in the root directory for the full license text.
 */

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::complexity)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! # Overview
//!
//! A library to process and analyze binary files containing
//! raw data from TRNGs with multiple ring oscillators (RO). 
//!
//! # Example
//!
//! ```
//! use std::path::PathBuf;
//! use rosa::splitter::RoSplitter;
//!
//!
//! const BINARY_FILE: &str = "/path/to/file.txt";
//! const NUM_ROS: usize = 16;
//!
//! fn main() {
//!     // Create instance of RoSplitter
//!     let split = RoSplitter::new(PathBuf::from(BINARY_FILE), NUM_ROS);
//! }

/// Module containing enums for each module-specific error types
pub mod error;
/// Module for splitting and combining binary files
pub mod splitter;
