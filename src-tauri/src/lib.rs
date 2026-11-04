// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde_json::json; // There was also Map in here, but considered as unsused.
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use std::path::PathBuf;

#[path = "parsers/legacy.rs"] mod plegacy;
#[path = "parsers/latest.rs"] mod platest;
#[path = "builders/mod.rs"] mod bmod;
#[path = "builders/cpack.rs"] mod bcpack;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
