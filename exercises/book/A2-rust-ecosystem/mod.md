
# Title: Build Your Powerful Rust Utility Library!

## Description:
Create and publish your own Rust crate offering valuable tools for common operations like string manipulation, file I/O, and data processing. Demonstrate your expertise in the Rust ecosystem by effectively utilizing modules, integrating external crates, and successfully publishing on crates.io.




Within your project directory, create three modules:
`string_utils`: Houses functions for string manipulation (reversing, case conversion, etc.).
`file_ops:` Contains methods for file reading and writing.
`data_processing:` Implements data sorting and filtering capabilities.

### String Power (string_utils):

Implement at least two string manipulation functions. Here are some ideas:
reverse(text: &str) -> String: Reverses a given string.
to_uppercase(text: &str) -> String: Converts all lowercase letters to uppercase (and vice versa with to_lowercase).
replace_all(text: &str, old_sub: &str, new_sub: &str) -> String: Replaces all occurrences of a substring with another.
Feel free to add more based on your interests and needs!(bonus)
File Magic (file_ops):

### Craft functions for file interaction:
read_file(path: &str) -> Result<String, String>: Reads file contents gracefully, handling errors.
write_file(path: &str, content: &str) -> Result<(), String>: Writes text to a file, catching any issues.
Consider adding methods for appending content, creating new files, etc(bonus)
Data Wrangling (data_processing):

### Show your data mastery:
sort_data(data: &mut Vec<T>, field: &str) -> Result<(), String>: Sorts a list based on a specified field (replace T with your data type).
filter_data(data: &[T], predicate: fn(T) -> bool) -> Vec<T>: Filters data based on a custom condition you define.



### Find Your Helper:

Head to crates.io and search for crates that can enrich your library's functionality. For example:
regex for advanced string processing
serde for data serialization/deserialization
rand for random number generation
Choose a crate that aligns with your project's goals.

### Bonus Round: Reuse your publish crate in another new Project. 
