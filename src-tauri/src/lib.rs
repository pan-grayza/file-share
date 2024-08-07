// Modules
mod types;

//Uses
use rfd::FileDialog;
// use serde_json::json;
use std::fs;
use std::fs::*;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use types::{Error, FileError, LinkedPath};
use warp::Filter;

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

#[tauri::command]
async fn start_file_server(
    stop_tx: tauri::State<'_, Arc<Mutex<Option<oneshot::Sender<()>>>>>,
) -> Result<String, String> {
    let dir_path = PathBuf::from("C:\\Users\\pasna\\Stepan\\Code");

    if !dir_path.exists() || !dir_path.is_dir() {
        return Err("Directory does not exist".into());
    }

    let dir_filter = warp::fs::dir(dir_path);
    let routes = warp::get().and(dir_filter);

    let mut port = 8000;
    loop {
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

        // Create a new oneshot channel for server shutdown
        let (new_stop_tx, stop_rx) = oneshot::channel();
        let mut stop_tx_lock = stop_tx.lock().unwrap();
        *stop_tx_lock = Some(new_stop_tx);

        // // Create the server with graceful shutdown
        // let (bound_addr, server_future) =
        //     warp::serve(routes.clone()).bind_with_graceful_shutdown(addr, async {
        //         stop_rx.await.ok();
        //     });

        // Attempt to bind the server
        match warp::serve(routes.clone()).try_bind_with_graceful_shutdown(addr, async {
            stop_rx.await.ok();
        }) {
            Ok((bound_addr, server_future)) => {
                // Spawn the server in a separate task
                tokio::spawn(async move { server_future.await });

                return Ok(format!("Server started at http://{}", addr));
            }
            Err(e) => {
                eprintln!("Failed to bind to port {}: {:?}", port, e);
                port += 1; // Increment the port and try again
                continue; // Try the next port
            }
        }
    }
}

#[tauri::command]
async fn stop_file_server(
    stop_tx: tauri::State<'_, Arc<Mutex<Option<oneshot::Sender<()>>>>>,
) -> Result<String, String> {
    let mut stop_tx_lock = stop_tx.lock().unwrap();
    if let Some(tx) = stop_tx_lock.take() {
        tx.send(())
            .map_err(|_| "Failed to send stop signal".to_string())?;
        println!("Im stopped!");
        Ok("Server stopped".to_string())
    } else {
        println!("Im not stopped!");
        Err("Server is not running".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let stop_tx: Arc<Mutex<Option<oneshot::Sender<()>>>> = Arc::new(Mutex::new(None));
    tauri::Builder::default()
        .manage(stop_tx)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            select_directory,
            link_directory,
            remove_directory,
            get_linked_paths,
            start_file_server,
            stop_file_server
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
