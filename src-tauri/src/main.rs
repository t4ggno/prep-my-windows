#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if let Err(error) = prep_my_windows_lib::run() {
        eprintln!("Application runtime failed: {error}");
        std::process::exit(1);
    }
}
