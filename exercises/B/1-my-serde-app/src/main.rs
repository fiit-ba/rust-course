//! Adapted from https://github.com/ferrous-systems/teaching-material/blob/main/assignments/serde-lifetimes.adoc
use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Deserialize, Serialize, Debug)]
struct BlogPost {
    id: u32,
    title: String,
}

fn main() -> anyhow::Result<()> {
    let data = fetch_data();
    let post: BlogPost = match serde_json::from_str(&data) {
        Ok(blog) => blog,
        Err(err) => {
            println!("error = {:?}", err);
            exit(1)
        }
    };

    println!("deserialized = {:?}", post);

    let post_json: String = serde_json::to_string(&post)?;
    println!("serialized = {:?}", post_json);

    let post_json: String = match serde_json::to_string(&post) {
        Ok(json) => json.to_string(),
        Err(err) => {
            println!("error = {:?}", err);
            exit(1)
        }
    };
    println!("serialized = {:?}", post_json);

    Ok(())
}

/// pretend that we call an API and get a JSON String back
fn fetch_data() -> String {
    String::from(
        r#"
            {
                "id": 1,
                "title": "Hello, Rust"
            }
        "#,
    )
}
