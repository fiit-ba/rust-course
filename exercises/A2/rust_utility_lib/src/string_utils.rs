//! String manipulation utilities
//!
//! This module provides various functions for common string operations.

/// Reverses a given string
///
/// # Examples
///
/// ```
/// use rust_utility_lib::string_utils::reverse;
///
/// assert_eq!(reverse("hello"), "olleh");
/// assert_eq!(reverse("rust"), "tsur");
/// ```
pub fn reverse(text: &str) -> String {
    text.chars().rev().collect()
}

/// Converts all lowercase letters to uppercase
///
/// # Examples
///
/// ```
/// use rust_utility_lib::string_utils::to_uppercase;
///
/// assert_eq!(to_uppercase("hello world"), "HELLO WORLD");
/// ```
pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// Converts all uppercase letters to lowercase
///
/// # Examples
///
/// ```
/// use rust_utility_lib::string_utils::to_lowercase;
///
/// assert_eq!(to_lowercase("HELLO WORLD"), "hello world");
/// ```
pub fn to_lowercase(text: &str) -> String {
    text.to_lowercase()
}

/// Replaces all occurrences of a substring with another
///
/// # Examples
///
/// ```
/// use rust_utility_lib::string_utils::replace_all;
///
/// assert_eq!(
///     replace_all("hello world hello", "hello", "hi"),
///     "hi world hi"
/// );
/// ```
pub fn replace_all(text: &str, old_sub: &str, new_sub: &str) -> String {
    text.replace(old_sub, new_sub)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("a"), "a");
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(to_uppercase("hello"), "HELLO");
        assert_eq!(to_lowercase("WORLD"), "world");
    }

    #[test]
    fn test_replace_all() {
        assert_eq!(replace_all("abc abc", "abc", "xyz"), "xyz xyz");
    }
}
