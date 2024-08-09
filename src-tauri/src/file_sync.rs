use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

#[tauri::command]
pub fn watch_directory() {
    let path = PathBuf::from("C:\\Users\\pasna\\Stepan\\Code");
    // Create a channel to receive the debounced events.
    let (tx, rx) = channel();

    // Create a debouncer with a debounce duration of 1 second.
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx).unwrap();

    // Watch the provided directory recursively.
    debouncer
        .watcher()
        .watch(&path, RecursiveMode::Recursive)
        .unwrap();

    println!("Watching directory: C:\\Users\\pasna\\Stepan\\Code");

    // This loop listens for debounced events.
    loop {
        match rx.recv() {
            Ok(events) => {
                for event in events.unwrap() {
                    match event.kind {
                        notify::EventKind::Create(_) => {
                            println!("File created: {:?}", event.paths);
                        }
                        notify::EventKind::Modify(_) => {
                            println!("File modified: {:?}", event.paths);
                        }
                        notify::EventKind::Remove(_) => {
                            println!("File removed: {:?}", event.paths);
                        }
                        _ => (),
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
