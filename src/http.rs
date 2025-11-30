use warp::Filter;
use crate::state::AppState;
use crate::p2p::NodeCommand;
use tokio::sync::mpsc;
use std::net::SocketAddr;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};

pub async fn start_server(
    port: u16, 
    state: AppState, 
    log_tx: mpsc::UnboundedSender<NodeCommand>
) {
    let state_filter = warp::any().map(move || state.clone());
    let log_tx_filter = warp::any().map(move || log_tx.clone());

    // GET /api/state
    let state_route = warp::path!("api" / "state")
        .and(warp::get())
        .and(state_filter.clone())
        .map(|state: AppState| {
            warp::reply::json(&state.snapshot())
        });

    // POST /api/log
    let log_route = warp::path!("api" / "log")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::bytes())
        .and(log_tx_filter.clone())
        .map(|bytes: bytes::Bytes, tx: mpsc::UnboundedSender<NodeCommand>| {
            let msg = String::from_utf8_lossy(&bytes).to_string();
            if let Err(e) = tx.send(NodeCommand::Log(msg)) {
                eprintln!("Failed to send log to P2P loop: {}", e);
                return warp::reply::with_status("Internal Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR);
            }
            warp::reply::with_status("Logged", warp::http::StatusCode::OK)
        });

    // GET / -> serve static files from ./web
    let static_files = warp::fs::dir("web");
    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("web/index.html"));

    // POST /api/dm
    #[derive(serde::Deserialize)]
    struct DmPayload {
        to: String,
        content: String,
    }

    let dm_route = warp::path!("api" / "dm")
        .and(warp::post())
        .and(warp::body::json())
        .and(log_tx_filter.clone())
        .map(|payload: DmPayload, tx: mpsc::UnboundedSender<NodeCommand>| {
            if let Err(e) = tx.send(NodeCommand::SendDm { to: payload.to, content: payload.content }) {
                eprintln!("Failed to send DM command: {}", e);
                return warp::reply::with_status("Internal Error", warp::http::StatusCode::INTERNAL_SERVER_ERROR);
            }
            warp::reply::with_status("Sent", warp::http::StatusCode::OK)
        });

    // WebSocket /ws
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(state_filter.clone())
        .map(|ws: warp::ws::Ws, state: AppState| {
            ws.on_upgrade(move |socket| handle_ws_connection(socket, state))
        });

    let routes = state_route
        .or(log_route)
        .or(dm_route)
        .or(ws_route)
        .or(index)
        .or(static_files);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Web Dashboard running at http://0.0.0.0:{}", port);

    warp::serve(routes).run(addr).await;
}

async fn handle_ws_connection(ws: WebSocket, state: AppState) {
    let (mut user_ws_tx, mut _user_ws_rx) = ws.split();
    let mut rx = state.telemetry_tx.subscribe();

    while let Ok(event) = rx.recv().await {
        if let Ok(json) = serde_json::to_string(&event) {
            println!("DEBUG: Sending WS message: {}", json); // Force stdout log
            if let Err(e) = user_ws_tx.send(Message::text(json)).await {
                eprintln!("WebSocket send error: {}", e);
                break;
            }
        }
    }
}
