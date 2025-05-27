//! Data processing utilities
//!
//! This module provides functions for sorting and filtering generic DataRecord
//! structure.

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// A generic data structure for demonstration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataRecord {
    pub id: u32,
    pub name: String,
    pub value: f64,
    pub active: bool,
}

impl DataRecord {
    pub fn new(id: u32, name: String, value: f64, active: bool) -> Self {
        Self {
            id,
            name,
            value,
            active,
        }
    }
}

/// Sorts a vector of DataRecord by a specified field
///
/// # Arguments
///
/// * `data` - Mutable reference to vector of DataRecord
/// * `field` - The field to sort by ("id", "name", "value")
///
/// # Returns
///
/// * `Ok(())` - If sorting was successful
/// * `Err(String)` - If the field name is invalid
///
/// # Examples
///
/// ```
/// use rust_utility_lib::data_processing::{sort_data_records, DataRecord};
///
/// let mut records = vec![
///     DataRecord::new(3, "Charlie".to_string(), 1.5, true),
///     DataRecord::new(1, "Alice".to_string(), 2.5, false),
///     DataRecord::new(2, "Bob".to_string(), 0.5, true),
/// ];
///
/// sort_data_records(&mut records, "id").unwrap();
/// assert_eq!(records[0].id, 1);
/// assert_eq!(records[1].id, 2);
/// assert_eq!(records[2].id, 3);
/// ```
pub fn sort_data_records(data: &mut Vec<DataRecord>, field: &str) -> Result<(), String> {
    match field {
        "id" => data.sort_by(|a, b| a.id.cmp(&b.id)),
        "name" => data.sort_by(|a, b| a.name.cmp(&b.name)),
        "value" => data.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Equal)),
        _ => return Err(format!("Invalid field name: {}", field)),
    }
    Ok(())
}

/// Filters a vector based on a predicate function
///
/// # Examples
///
/// ```
/// use rust_utility_lib::data_processing::filter_data;
///
/// let numbers = vec![1, 2, 3, 4, 5, 6];
/// let evens = filter_data(&numbers, |&x| x % 2 == 0);
/// assert_eq!(evens, vec![2, 4, 6]);
/// ```
pub fn filter_data<T: Clone>(data: &[T], predicate: fn(&T) -> bool) -> Vec<T> {
    data.iter()
        .filter(|&item| predicate(item))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_data_records() {
        let mut records = vec![
            DataRecord::new(3, "Charlie".to_string(), 1.5, true),
            DataRecord::new(1, "Alice".to_string(), 2.5, false),
            DataRecord::new(2, "Bob".to_string(), 0.5, true),
        ];

        sort_data_records(&mut records, "id").unwrap();
        assert_eq!(records[0].id, 1);
        assert_eq!(records[1].id, 2);
        assert_eq!(records[2].id, 3);
    }
}
