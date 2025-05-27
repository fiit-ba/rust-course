//! # Rust Utility Library
//!
//! A comprehensive utility library providing tools for string manipulation,
//! file operations, and data processing.
//!
//! ## Modules
//!
//! - [`string_utils`] - String manipulation functions
//! - [`file_ops`] - File I/O operations
//! - [`data_processing`] - Data sorting and filtering utilities

pub mod data_processing;
pub mod file_ops;
pub mod string_utils;

// Re-export commonly used items
pub use data_processing::*;
pub use file_ops::*;
pub use string_utils::*;
