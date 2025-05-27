use std::ffi::CString;

// Argument forwarding
//
// see README for instructions

fn main() {
    let mut path = std::env::current_exe().unwrap();
    path.set_file_name("log");

    let executable = CString::new(path.to_str().unwrap()).unwrap();

    // Collect all arguments, but replace the first one with our executable path
    let mut args: Vec<CString> = std::env::args()
        .skip(1) // Skip the original program name
        .map(|arg| CString::new(arg).unwrap())
        .collect();

    // Insert executable path as first argument
    args.insert(0, executable.clone());

    // Create argv array with null terminator
    let mut argv: Vec<*const i8> = args.iter().map(|s| s.as_ptr()).collect();
    argv.push(std::ptr::null());

    // Collect all environment variables
    let env_vars: Vec<CString> = std::env::vars()
        .map(|(key, value)| CString::new(format!("{}={}", key, value)).unwrap())
        .collect();

    // Create envp array with null terminator
    let mut envp: Vec<*const i8> = env_vars.iter().map(|s| s.as_ptr()).collect();
    envp.push(std::ptr::null());

    unsafe { libc::execve(executable.as_ptr(), argv.as_ptr(), envp.as_ptr()) };

    // if control flow ever gets here, the execve call failed.
    println!("{:#?}", std::io::Error::last_os_error());
}
