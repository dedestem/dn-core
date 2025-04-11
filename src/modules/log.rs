use std::fs::{OpenOptions, rename, create_dir_all};
use std::io::{Write, BufRead, BufReader};
use std::process::exit;
use std::sync::Mutex;
use std::path::Path;

use chrono::Local;
use once_cell::sync::Lazy;

const LOG_DIR: &str = "logs";
const LOG_FILE: &str = "logs/latest.log";

static LOGGER: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
    rotate_old_log_if_exists();
    create_dir_all(LOG_DIR).expect("Failed to create logs directory");

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Failed to open log file");

    Mutex::new(file)
});

// Uses the extract_timestamp function to rename the old log file
fn rotate_old_log_if_exists() {
    if Path::new(LOG_FILE).exists() {
        if let Ok(file) = std::fs::File::open(LOG_FILE) {
            let reader = BufReader::new(file);

            // Try to read the first line and extract the timestamp
            if let Some(Ok(line)) = reader.lines().next() {
                if let Some(timestamp) = extract_timestamp(&line) {
                    let filename = format!("{}/{}.log", LOG_DIR, timestamp.replace(['[', ']', ':', ' '], "-"));
                    let _ = rename(LOG_FILE, filename);
                }
            }
        }
    }
}

// Extracts the timestamp from the first line of the log file.
// So we can call the log file after that date and time.
fn extract_timestamp(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split(']').collect();
    if parts.len() > 1 {
        return Some(parts[0].trim_start_matches('[').to_string());
    }
    None
}

pub fn log(level: &str, msg: &str) {
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]");
    let log_line = format!("{} [{}] {}\n", timestamp, level.to_uppercase(), msg);

    let mut file = LOGGER.lock().unwrap();
    let _ = file.write_all(log_line.as_bytes());

    if level == "error" {
        exit(1);
    }
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log::log("trace", &format!($($arg)*));
    };
}
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log::log("info", &format!($($arg)*));
    };
}
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::log("warn", &format!($($arg)*));
    };
}
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log::log("error", &format!($($arg)*));
    };
}
