use std::time::Instant;

mod modules;
use modules::enviroment;
use modules::assets;
use modules::log;
use modules::dependencies;

fn main() {
    println!("View logs/latest.log for logs...");
    let current_dir = enviroment::get_env_dir();
    info!("ENV DIR: {:?}", current_dir);

    info!("Starting assets checks");
    let start_time_assets_checking = Instant::now();
    assets::check_assets();
    let elapsed_time_assets_checking = start_time_assets_checking.elapsed();
    info!("Assets checking took: {:?}", elapsed_time_assets_checking);

    info!("Starting Depedencies Check");
    let start_time_depedencies_checking = Instant::now();
    dependencies::check_dependencies();
    let elapsed_time_depedencies_checking = start_time_depedencies_checking.elapsed();
    info!("Depedencies checking took: {:?}", elapsed_time_depedencies_checking);
}
