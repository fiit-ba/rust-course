use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// An imaginary config file
#[derive(Serialize, Deserialize, Debug)]
pub struct Config<'a> {
    port: u16,
    base_url: &'a str,
    s3_path: &'a str,
    database_url: &'a str,
}

#[derive(Debug)]
/// Config deserialization error
pub enum Error {
    /// Something went wrong deserializing JSON
    Json(serde_json::Error),
    /// Something went wrong deserializing YAML
    Yaml(serde_yaml::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Json(e) => write!(f, "JSON deserialization error: {}", e),
            Error::Yaml(e) => write!(f, "YAML deserialization error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::Yaml(err)
    }
}

trait DeserializeConfig {
    /// Deserialize the contents into a `Config`
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error>;
}

// JSON deserializer
struct JsonDeserializer;

impl DeserializeConfig for JsonDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        let config: Config<'a> = serde_json::from_str(contents)?;
        Ok(config)
    }
}

/// YAML deserializer
struct YamlDeserializer;

impl DeserializeConfig for YamlDeserializer {
    fn deserialize<'a>(&self, contents: &'a str) -> Result<Config<'a>, Error> {
        let config: Config<'a> = serde_yaml::from_str(contents)?;
        Ok(config)
    }
}

/// Get the appropriate deserializer based on file extension
fn get_deserializer(extension: Option<&str>) -> Result<Box<dyn DeserializeConfig>, String> {
    match extension {
        Some("json") => Ok(Box::new(JsonDeserializer)),
        Some("yml") | Some("yaml") => Ok(Box::new(YamlDeserializer)),
        Some(ext) => Err(format!("Unsupported file extension: {}", ext)),
        None => Err("No file extension found".to_string()),
    }
}

fn main() {
    let mut args = std::env::args();
    // Unwrapping is OK here, as UTF-8 Strings can always be converted to PathBufs
    let Some(path) = args.nth(1).map(|a| PathBuf::try_from(a).unwrap()) else {
        eprintln!("Please specify the input path");
        return;
    };
    // Unwrapping is Ok as `path` was created from UTF-8 string, and so is the extension
    let extension = path.extension().map(|o| o.to_str().unwrap());
    let file_contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            // `path` was created from an UTF-8 string, so can be converted to one
            eprintln!(
                "Error reading file at path {}: {}",
                path.to_str().unwrap(),
                e
            );
            return;
        }
    };

    let deserializer = match get_deserializer(extension) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error getting deserializer: {}", e);
            return;
        }
    };

    let config: Config = match deserializer.deserialize(&file_contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error deserializing config: {}", e);
            return;
        }
    };

    println!("Config was: {config:?}");
}
