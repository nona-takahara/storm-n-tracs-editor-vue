// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_derive::{Deserialize, Serialize};
use std::{self, fs::File, io::Write, path::PathBuf};
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Serialize, Deserialize, Debug)]
struct PathConfig {
    sw_tile_path: String,
    addon_path: String,
}
impl Default for PathConfig {
    fn default() -> Self {
        Self {
            sw_tile_path:
                "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Stormworks\\rom\\data\\tiles\\"
                    .to_string(),
            addon_path: "C:\\".to_string(),
        }
    }
}

#[tauri::command]
fn read_tile_file_command(filename: String) -> Result<String, String> {
    let cfg = confy::load::<PathConfig>("storm-n-tracs-editor");
    match cfg {
        Ok(cfg) => {
            let dir = std::path::Path::new(&cfg.sw_tile_path);
            let filename = std::path::Path::new(&filename).file_name();
            match filename {
                Some(filename) => {
                    let content = match std::fs::read_to_string(dir.join(filename)) {
                        Ok(content) => content,
                        Err(e) => return Err(e.to_string()),
                    };
                    return Ok(content);
                }
                None => Err("Cannot get settings".to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
fn read_addon_command(foldername: String) -> Result<String, String> {
    let cfg = confy::load::<PathConfig>("storm-n-tracs-editor");
    match cfg {
        Ok(cfg) => {
            let dir = std::path::Path::new(&cfg.addon_path);
            let filename = std::path::Path::new(&foldername).file_stem();
            match filename {
                Some(filename) => {
                    let content = match std::fs::read_to_string(dir.join(filename).join("playlist.xml")) {
                        Ok(content) => content,
                        Err(e) => return Err(e.to_string()),
                    };
                    return Ok(content);
                }
                None => Err("Cannot get settings".to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    }
}

#[tauri::command]
fn open_file_command() -> Result<String, String> {
    let path: Option<PathBuf> = FileDialogBuilder::new()
        .add_filter("JSON file", &["json"])
        .pick_file();
    match path {
        Some(filepath) => {
            let content = match std::fs::read_to_string(filepath) {
                Ok(content) => content,
                Err(e) => return Err(e.to_string()),
            };
            return Ok(content);
        }
        None => return Err("File select canceld.".to_string()),
    };
}

#[tauri::command]
fn save_file_command(save_value: String) -> Result<(), String> {
    let path: Option<PathBuf> = FileDialogBuilder::new()
        .add_filter("JSON file", &["json"])
        .save_file();
    match path {
        Some(filepath) => {
            let file = File::create(filepath);
            match file {
                Ok(mut fs) => {
                    let write = fs.write_all(save_value.as_bytes());
                    match write {
                        Ok(_) => return Ok(()),
                        Err(e) => return Err(e.to_string()),
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        None => return Err("File select canceld.".to_string()),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_file_command,
            read_addon_command,
            read_tile_file_command,
            open_file_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
