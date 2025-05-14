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
//! use rosa::RoSplitter;
//!
//!
//! const BINARY_FILE: &str = "/path/to/file.txt";
//! const NUM_ROS: usize = 16;
//!
//! fn main() {
//!     // Create instance of RoSplitter
//!     let split = RoSplitter::new(BINARY_FILE, NUM_ROS);
//! }

/// Module containing enums for each module-specific error types
pub mod error;
/// Module for splitting and combining binary files
pub mod splitter;
