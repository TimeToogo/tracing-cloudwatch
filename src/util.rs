use std::env;

pub fn print_debug(msg: String) {
    if env::var("SEGFAULTAI_DEBUG").is_ok() {
        eprintln!("[tracing-cloudwatch] {}", msg);
    }
}
