use std::path::Path;
use crate::{trace, error};

fn check_asset(asset_path: &str) -> bool {
    let path = Path::new(asset_path);

    if path.exists() {
        if path.is_file() {
            trace!("Asset {:?} found!", asset_path);
        } else if path.is_dir() {
            trace!("Asset folder {:?} found!", asset_path);
        }
        true
    } else {
        error!("Asset {:?} does not exist.", asset_path);
        false
    }
}

pub fn check_assets() {
    let assets = ["install/install_docker_compose.sh", "install/install_docker.sh"]; // Folders and files are supported

    for asset in assets.iter() {
        check_asset(asset);
    }
}
