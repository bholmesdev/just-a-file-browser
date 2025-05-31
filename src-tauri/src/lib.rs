// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn list_desktop_files() -> Result<Vec<String>, String> {
    // Get the user's home directory
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    
    // Construct path to Desktop
    let desktop_path = home_dir.join("Desktop");
    
    // Read directory contents
    let entries = fs::read_dir(&desktop_path)
        .map_err(|e| format!("Failed to read Desktop directory: {}", e))?;
    
    // Collect file names
    let mut file_names = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Error reading directory entry: {}", e))?;
        if let Some(name) = entry.file_name().to_str() {
            file_names.push(name.to_string());
        }
    }
    
    Ok(file_names)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, list_desktop_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
