//! You can't change anything except adding or removing references.

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&mut data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();

    println!("{}", data);
}
