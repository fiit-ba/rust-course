//! File operation utilities
//!
//! This module provides safe and functions for file I/O operations
//! including reading, writing, and appending to files.

use std::io::Write;
use std::{fs, io};

/// Reads file contents and returns them as a String
///
/// # Arguments
///
/// * `path` - The path to the file to read
///
/// # Returns
///
/// * `Ok(String)` - The file contents if successful
/// * `Err(String)` - An error message if the operation fails
///
/// # Examples
///
/// ```no_run
/// use rust_utility_lib::file_ops::read_file;
///
/// match read_file("example.txt") {
///     Ok(content) => println!("File content: {}", content),
///     Err(e) => eprintln!("Error reading file: {}", e),
/// }
/// ```
pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Writes content to a file, creating it if it doesn't exist
///
/// # Arguments
///
/// * `path` - The path to the file to write
/// * `content` - The content to write to the file
///
/// # Returns
///
/// * `Ok(())` - If the write operation was successful
/// * `Err(String)` - An error message if the operation fails
///
/// # Examples
///
/// ```no_run
/// use rust_utility_lib::file_ops::write_file;
///
/// match write_file("output.txt", "Hello, World!") {
///     Ok(()) => println!("File written successfully"),
///     Err(e) => eprintln!("Error writing file: {}", e),
/// }
/// ```
pub fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Appends content to an existing file
///
/// # Arguments
///
/// * `path` - The path to the file to append to
/// * `content` - The content to append to the file
///
/// # Returns
///
/// * `Ok(())` - If the append operation was successful
/// * `Err(String)` - An error message if the operation fails
///
/// # Examples
///
/// ```no_run
/// use rust_utility_lib::file_ops::append_file;
///
/// match append_file("log.txt", "\nNew log entry") {
///     Ok(()) => println!("Content appended successfully"),
///     Err(e) => eprintln!("Error appending to file: {}", e),
/// }
/// ```
pub fn append_file(path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    file.write_all(content.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_and_read_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let file_path_str = file_path.to_str().unwrap();

        let content = "Hello, World!";

        // Test writing
        assert!(write_file(file_path_str, content).is_ok());

        // Test reading
        let read_content = read_file(file_path_str).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_append_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("append_test.txt");
        let file_path_str = file_path.to_str().unwrap();

        // Write initial content
        write_file(file_path_str, "Hello").unwrap();

        // Append content
        append_file(file_path_str, ", World!").unwrap();

        // Read and verify
        let content = read_file(file_path_str).unwrap();
        assert_eq!(content, "Hello, World!");
    }
}
