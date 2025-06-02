// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FileInfo {
    name: String,
    created: Option<u64>,  // Unix timestamp in seconds
    modified: Option<u64>, // Unix timestamp in seconds
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_desktop_files() -> Result<Vec<FileInfo>, String> {
    // Get the user's home directory
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    
    // Construct path to Desktop
    let desktop_path = home_dir.join("Desktop");
    
    // Read directory contents
    let entries = fs::read_dir(&desktop_path)
        .map_err(|e| format!("Failed to read Desktop directory: {}", e))?;
    
    // Collect file info with metadata
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        if let Some(name) = entry.file_name().to_str() {
            let metadata = entry.metadata()
                .map_err(|e| format!("Failed to read metadata for {}: {}", name, e))?;
            
            let created = metadata.created()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs());
            
            let modified = metadata.modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs());
            
            files.push(FileInfo {
                name: name.to_string(),
                created,
                modified,
            });
        }
    }
    
    Ok(files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_desktop_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
