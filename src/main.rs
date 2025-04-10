//? STD Libaries
use std::env;
use std::time::Instant;

//? Modules
mod modules;
use modules::log;
pub use modules::assets;

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    info!("ENV DIR: {:?}", current_dir);

    let start_time_asset_checking = Instant::now();
    assets::check_assets();
    let elapsed_time_asset_checking = start_time_asset_checking.elapsed();
    info!("Asset checking took: {:?}", elapsed_time_asset_checking);

    //let mut _child = Command::new("deno")
    //    .args(["task", "run"])
    //    .spawn()
    //    .expect("Failed to start Deno");
}
