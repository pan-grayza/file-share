use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::sync::oneshot;
use tokio::time::{Duration, Instant};
use warp::ws::{Message, WebSocket};
use warp::Filter;

const BROADCAST_ADDRESS: &str = "255.255.255.255:8080";
const DISCOVERY_PORT: u16 = 8080;
// let DEVICE_ID: String = tauri_plugin_os::hostname();
// println!("{}", DEVICE_ID);

const PING_SERVER: &str = "8.8.8.8:80"; // Example server for latency measurement

// async fn measure_latency() -> Result<u64, Box<dyn std::error::Error>> {
//     let start = Instant::now();
//     let socket = UdpSocket::bind("0.0.0.0:0").await?;
//     socket.connect(PING_SERVER).await?;

//     let message = b"ping";
//     socket.send(message).await?;

//     let mut buf = [0; 1024];
//     socket.recv_from(&mut buf).await?;

//     let elapsed = start.elapsed().as_millis() as u64;
//     Ok(elapsed)
// }

// async fn broadcast_announcement(latency: u64) -> Result<(), Box<dyn std::error::Error>> {
//     let socket = UdpSocket::bind("0.0.0.0:0").await?;
//     socket.set_broadcast(true).await?;

//     let message = format!("ID: {} Latency: {}", DEVICE_ID, latency).as_bytes();
//     let addr = BROADCAST_ADDRESS.parse::<SocketAddr>()?;

//     loop {
//         socket.send_to(message, addr).await?;
//         sleep(Duration::from_secs(5)).await; // Broadcast every 5 seconds
//     }
// }

#[tauri::command]
pub async fn start_file_server(
    stop_tx: tauri::State<'_, Arc<Mutex<Option<oneshot::Sender<()>>>>>,
) -> Result<String, String> {
    let mut port = 8000;
    loop {
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();

        // Create a new oneshot channel for server shutdown
        let (new_stop_tx, stop_rx) = oneshot::channel();
        let mut stop_tx_lock = stop_tx.lock().unwrap();
        *stop_tx_lock = Some(new_stop_tx);

        // Define WebSocket route
        let ws_route = warp::path("ws")
            .and(warp::ws())
            .map(|ws: warp::ws::Ws| ws.on_upgrade(handle_ws_connection));

        // Try to bind the server to the address
        match warp::serve(ws_route.clone()).try_bind_with_graceful_shutdown(addr, async {
            stop_rx.await.ok();
        }) {
            Ok((bound_addr, server_future)) => {
                // Spawn the server in a separate task
                tokio::spawn(async move {
                    let _ = server_future.await;
                });

                return Ok(format!("WebSocket server started at ws://{}", bound_addr));
            }
            Err(e) => {
                eprintln!("Failed to bind to port {}: {:?}", port, e);
                port += 1; // Increment the port and try again
                continue; // Try the next port
            }
        }
    }
}

// Handle WebSocket connection
async fn handle_ws_connection(ws: WebSocket) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(message) => {
                if message.is_text() {
                    let msg = message.to_str().unwrap();
                    println!("Received message: {}", msg);

                    // Example: Echo the message back to the client
                    if let Err(e) = ws_tx.send(Message::text(msg)).await {
                        eprintln!("Error sending message: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {}", e);
                break;
            }
        }
    }
}

#[tauri::command]
pub async fn stop_file_server(
    stop_tx: tauri::State<'_, Arc<Mutex<Option<oneshot::Sender<()>>>>>,
) -> Result<String, String> {
    let mut stop_tx_lock = stop_tx.lock().unwrap();
    if let Some(tx) = stop_tx_lock.take() {
        tx.send(())
            .map_err(|_| "Failed to send stop signal".to_string())?;
        println!("WebSocket server stopped!");
        Ok("Server stopped".to_string())
    } else {
        println!("WebSocket server is not running!");
        Err("Server is not running".to_string())
    }
}
