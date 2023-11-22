// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod error;
pub mod fda;

use error::Result;

#[tauri::command]
fn has_full_disk() -> bool {
    dbg!(fda::DiskAccess::has_fda());
    fda::DiskAccess::has_fda()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![has_full_disk])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
