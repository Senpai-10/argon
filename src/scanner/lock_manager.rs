use std::env;
use std::path::PathBuf;

fn get_lock_file() -> PathBuf {
    env::temp_dir().join("argon-scanner-lock")
}

pub fn is_locked() -> bool {
    get_lock_file().exists()
}

pub fn lock() {
    let file = get_lock_file();

    if !file.exists() {
        _ = std::fs::write(file, "");
    }
}

pub fn unlock() {
    let file = get_lock_file();

    if file.exists() {
        _ = std::fs::remove_file(file);
    }
}
