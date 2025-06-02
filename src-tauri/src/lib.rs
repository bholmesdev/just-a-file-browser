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
fn list_desktop_files(search_query: String) -> Result<Vec<FileInfo>, String> {
    // Get the user's home directory
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    
    // Construct path to Desktop
    let desktop_path = home_dir.join("Desktop");
    
    // Read directory contents
    let entries = fs::read_dir(&desktop_path)
        .map_err(|e| format!("Failed to read Desktop directory: {}", e))?;
    
    // Collect file info with metadata
    let mut files = Vec::new();
    let search_lower = search_query.to_lowercase();
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        if let Some(name) = entry.file_name().to_str() {
            // Filter by search query if provided
            if !search_query.is_empty() && !name.to_lowercase().contains(&search_lower) {
                continue;
            }
            
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
    
    // Sort by most recently modified (descending order)
    files.sort_by(|a, b| {
        match (a.modified, b.modified) {
            (Some(a_mod), Some(b_mod)) => b_mod.cmp(&a_mod), // Most recent first
            (Some(_), None) => std::cmp::Ordering::Less,     // Files with mod time come first
            (None, Some(_)) => std::cmp::Ordering::Greater,  // Files without mod time come last
            (None, None) => a.name.cmp(&b.name),             // Fallback to name sorting
        }
    });
    
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
