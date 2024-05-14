use std::time::SystemTime;
use std::io::{self, Write};

#[allow(dead_code)]
pub fn debug(msg: &str) {
    let now_msec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("🏁 {now_msec}: {msg}");
}

#[allow(dead_code)]
pub fn info(msg: &str) {
    let now_msec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("✅ {now_msec}: {msg}");
}

#[allow(dead_code)]
pub fn warn(msg: &str) {
    let now_msec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("🚧 {now_msec}: {msg}");
}

#[allow(dead_code)]
pub fn error(msg: &str) {
    let now_msec = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    println!("🚩 {now_msec}: {msg}");
}

#[allow(dead_code)]
pub fn progress(msg: &str) {
    print!("{msg}");
    match io::stdout().flush() {
        Ok(_) => {},
        Err(_) => {},
    }
}
