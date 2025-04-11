use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use crate::{error, trace};

pub fn get_env_dir() -> PathBuf {
    match env::current_dir() {
        Ok(dir) => {
            trace!("Current enviroment directory: {:?}", dir);
            dir
        },
        Err(e) => {
            error!("Failed to get current directory: {}", e);
            PathBuf::new() // Return an empty PathBuf as a fallback
        }
    }
}


pub fn is_root() -> bool {
    match Command::new("id").arg("-u").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim() == "0",
        Err(_) => false,
    }
}

pub fn is_debian_based() -> bool {
    fs::read_to_string("/etc/os-release")
        .map(|content| content.contains("ID_LIKE=debian") || content.contains("ID=debian"))
        .unwrap_or(false)
}