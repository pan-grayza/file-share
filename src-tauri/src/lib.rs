// Modules
mod file_sync;
mod local_dir_man;
mod server;
mod types;

//Uses
use file_sync::watch_directory;
use local_dir_man::{
    get_linked_paths, link_directory, remove_directory, select_directory, VAULT_PATH,
};
use server::{start_file_server, stop_file_server};
use std::fs::*;
use std::io::prelude::*;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use types::LinkedPath;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let stop_tx: Arc<Mutex<Option<oneshot::Sender<()>>>> = Arc::new(Mutex::new(None));
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .manage(stop_tx)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            select_directory,
            link_directory,
            remove_directory,
            get_linked_paths,
            start_file_server,
            stop_file_server,
            watch_directory
        ])
        .setup(|_app| {
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
