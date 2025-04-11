use std::process::{Command, Stdio};
use crate::{error, info, modules::enviroment::{is_debian_based, is_root}, trace};

pub fn check_dependencies() {
    if !is_root() {
        error!("DN-Core needs root access. Try running with sudo.");
        return;
    }

    if !is_debian_based() {
        error!("DN-Core only supports Debian-based systems.");
        return;
    }
    
    info!("Dependency enviroments checks OK.");

    if !is_installed("docker") {
        info!("Docker not found. Installing...");
        run_script("install/install_docker.sh");
        info!("Docker installed successfully.");
    } else {
        trace!("Docker is already installed. OK");
    }

    if !is_installed("docker-compose") {
        info!("Docker Compose not found. Installing...");
        run_script("install/install_docker_compose.sh");
        info!("Docker Compose installed successfully.");
    } else {
        trace!("Docker Compose is already installed. OK");
    }

    info!("Dependencies OK");
}



fn is_installed(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn run_script(path: &str) {
    let status = Command::new("bash")
        .arg(path)
        .status()
        .expect("Failed to run script");

    if !status.success() {
        error!("Script {} failed with exit code {}", path, status.code().unwrap_or(-1));
    }
}
