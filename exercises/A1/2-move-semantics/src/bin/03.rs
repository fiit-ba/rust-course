//! Make me compile without adding new lines-- just changing existing lines!

fn main() {
    let s0 = String::new();

    let mut s1 = append_to_string(s0);

    println!("{} == `{}`", stringify!(s1), s1);

    s1.push('!');

    println!("{} == `{}`", stringify!(s1), s1);
}

// declare `s` as mutable ownership
fn append_to_string(mut s: String) -> String {
    s.push_str("Hello World");

    s
}
