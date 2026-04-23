// Tauri 2 binary entry point
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    kryptopilot_lib::run()
}
