// Modules
mod types;

//Uses
use rfd::FileDialog;
// use serde_json::json;
use std::fs;
use std::fs::*;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use types::{Error, FileError, LinkedPath};

const VAULT_PATH: &str = "vault/paths.json";

fn open_and_read_path_file() -> Result<Vec<LinkedPath>, FileError> {
    let data = fs::read_to_string(VAULT_PATH).unwrap();
    let config_contents: Vec<LinkedPath> = serde_json::from_str(&data)?;
    Ok(config_contents)
}

#[tauri::command]
fn select_directory() -> Result<PathBuf, Error> {
    let selected_dir: Option<PathBuf> = FileDialog::new().set_directory(".").pick_folder();
    match selected_dir {
        Some(path) => {
            return Ok(path);
        }
        None => return Ok(PathBuf::from("")),
    }
}

#[tauri::command]
fn link_directory(path: String, name: String) -> Result<String, FileError> {
    // Open a file and read it contents
    let mut linked_paths = open_and_read_path_file().unwrap();
    if name == "" {
        return Ok("Name your vault".to_string());
    };
    if linked_paths.iter().any(|x| x.name == name) {
        return Ok("Vault with this name already exists".to_string());
    };
    if path == "" {
        return Ok("Directory not selected".to_string());
    };
    // Define new path
    let new_linked_path = LinkedPath {
        name: name,
        id: 1,
        path: PathBuf::from(path),
    };
    //Write new path
    linked_paths.push(new_linked_path);
    let data = serde_json::to_string(&linked_paths).unwrap();
    fs::write(VAULT_PATH, data).unwrap();

    Ok("Directory linked successfully".to_string())
}

#[tauri::command]
fn remove_directory(linked_path_name: String) -> Result<String, FileError> {
    // Open a file and read it contents
    let mut linked_paths = open_and_read_path_file().unwrap();
    //Delete LinkedPath
    linked_paths.retain(|path| path.name != linked_path_name);

    let data = serde_json::to_string(&linked_paths).unwrap();
    fs::write(VAULT_PATH, data).unwrap();

    Ok("Directory removed successfully".to_string())
}

#[tauri::command]
fn get_linked_paths() -> Result<Vec<LinkedPath>, FileError> {
    // Open a file and read it contents
    let linked_paths = open_and_read_path_file().unwrap();
    // println!("{}", linked_paths[0].name);

    Ok(linked_paths)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            select_directory,
            link_directory,
            remove_directory,
            get_linked_paths,
        ])
        .setup(|app| {
            // dbg!(scope.allowed());
            if !Path::new("vault").is_dir() {
                create_dir("vault")?;
            }
            if !Path::new(VAULT_PATH).exists() {
                let mut file = File::create(VAULT_PATH)?;
                let linked_paths =
                    serde_json::to_string_pretty::<Vec<LinkedPath>>(&vec![]).unwrap();

                file.write_all(linked_paths.as_bytes()).unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
