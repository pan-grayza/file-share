// Modules
mod types;

//Uses
use rfd::FileDialog;
use std::fs::*;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use types::{Error, LinkDirectoryError, LinkedPath};

const VAULT_PATH: &str = "vault/paths.toml";

fn open_and_read_file(path: &str) -> Result<String, Error> {
    let mut paths_config = File::open(path)?;
    let mut config_contents = String::new();
    paths_config.read_to_string(&mut config_contents)?;
    Ok(config_contents)
}

#[tauri::command]
fn select_directory() -> Result<PathBuf, Error> {
    let selected_dir: Option<PathBuf> = FileDialog::new().set_directory(".").pick_folder();
    match selected_dir {
        Some(path) => {
            println!("{}", path.display());
            return Ok(path);
        }
        None => return Ok(PathBuf::from("")),
    }
}

#[tauri::command]
fn link_directory(path: String, name: String) -> Result<String, LinkDirectoryError> {
    // Open a file and read it contents
    let config_contents = open_and_read_file(VAULT_PATH).unwrap();
    let mut linked_paths: Vec<LinkedPath> = toml::from_str(&config_contents)?;
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
    let toml_string = toml::to_string(&linked_paths)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(VAULT_PATH)?;
    file.write_all(toml_string.as_bytes())?;

    Ok("Directory linked successfully".to_string())
}

#[tauri::command]
fn remove_directory(linked_path_name: String) -> Result<String, LinkDirectoryError> {
    // Open a file and read it contents
    let config_contents = open_and_read_file(VAULT_PATH).unwrap();
    let mut linked_paths: Vec<LinkedPath> = toml::from_str(&config_contents)?;
    //Delete LinkedPath
    linked_paths.retain(|path| path.name != linked_path_name);
    let toml_string = toml::to_string(&linked_paths)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(VAULT_PATH)?;
    file.write_all(toml_string.as_bytes())?;

    Ok("Directory removed successfully".to_string())
}

#[tauri::command]
fn get_linked_paths() -> Result<Vec<LinkedPath>, LinkDirectoryError> {
    // Open a file and read it contents
    let config_contents = open_and_read_file(VAULT_PATH).unwrap();
    let linked_paths: Vec<LinkedPath> = toml::from_str(&config_contents)?;
    println!("{}", linked_paths[0].name);

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
            if !Path::new("vault/paths.toml").exists() {
                let linked_paths: Vec<LinkedPath> = vec![];
                let toml_string = toml::to_string(&linked_paths)?;
                let mut file = File::create("vault/paths.toml")?;
                file.write_all(toml_string.as_bytes())?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
