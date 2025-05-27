use std::os::raw::{c_uchar, c_uint};

extern "C" {
    fn CRC32(data: *const c_uchar, data_length: usize) -> c_uint;
}

fn crc32(data: &[u8]) -> u32 {
    unsafe { CRC32(data.as_ptr(), data.len()) as u32 }
}

fn main() {
    println!("{:#x}", crc32(b"12345678"));
}
