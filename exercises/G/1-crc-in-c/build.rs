extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=crc32.h");
    println!("cargo:rerun-if-changed=crc32.c");
    cc::Build::new().file("crc32.c").compile("crc32.a");
}
