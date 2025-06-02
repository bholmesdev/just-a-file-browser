// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::fs;

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

    // Define the directories to search
    let search_dirs = vec![
        home_dir.join("Desktop"),
        home_dir.join("Downloads"),
        home_dir.join("Documents"),
    ];

    // Collect file info with metadata from all directories
    let mut files = Vec::new();
    let search_lower = search_query.to_lowercase();

    for dir_path in search_dirs {
        if !dir_path.exists() {
            continue;
        }
        
        let entries = match fs::read_dir(&dir_path) {
            Ok(entries) => entries,
            Err(_) => continue, // Skip directories we can't read
        };
        
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue, // Skip entries we can't read
            };
            
            // Only process files, not directories
            if entry.path().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    // Filter by search query if provided
                    if !search_query.is_empty() && !name.to_lowercase().contains(&search_lower) {
                        continue;
                    }
                    
                    if let Ok(metadata) = entry.metadata() {
                        let created = metadata
                            .created()
                            .ok()
                            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|duration| duration.as_secs());
                        
                        let modified = metadata
                            .modified()
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
            }
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
