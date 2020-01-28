use std::process::exit;

/// Error out and exit
pub fn err(msg: &str, code: i32) -> ! {
    eprintln!("ERROR!: {}", msg);
    exit(code);
}

/// Wrapper around matching error for cleaner panics
pub fn handle<T, E>(res: Result<T, E>, msg: &str) -> T {
    match res {
        Ok(e) => e,
        Err(_) => {
            err(format!("{}", msg).as_str(), 1);
        }
    }
}
