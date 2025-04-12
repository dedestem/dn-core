use std::io::{self, Write};
use std::process::{Command, Stdio};

use crate::{error, info};

fn execute_command(command: &str, args: &[&str]) -> Result<(), String> {
    let output = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Error: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout); // Log the output
    Ok(())
}

fn start_container(container_name: &str) -> Result<(), String> {
    execute_command(
        "docker",
        &[
            "compose",
            "-f",
            &format!(
                "/etc/dn-core/containers/{}/docker-compose.yaml",
                container_name
            ),
            "up",
            "-d",
        ],
    )
}

fn stop_container(container_name: &str) -> Result<(), String> {
    execute_command(
        "docker",
        &[
            "compose",
            "-f",
            &format!(
                "/etc/dn-core/containers/{}/docker-compose.yaml",
                container_name
            ),
            "down",
        ],
    )
}

fn restart_container(container_name: &str) -> Result<(), String> {
    stop_container(container_name)?;
    start_container(container_name)
}

fn get_container_status(container_name: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .args(&["inspect", "--format", "'{{.State.Status}}'", container_name])
        .output()
        .map_err(|e| format!("Failed to check container status: {}", e))?;

    if !output.status.success() {
        return Err("Failed to retrieve container status.".to_string());
    }

    let status = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(status)
}

pub fn start_nginx() {
    let container_name = "nginx";

    match start_container(container_name) {
        Ok(_) => info!("{} container started successfully.", container_name),
        Err(e) => error!("Failed to start {}: {}", container_name, e),
    }

    // Example: Check container status
    match get_container_status(container_name) {
        Ok(status) => info!("Container status: {}", status),
        Err(e) => error!("Error getting status: {}", e),
    }
}
