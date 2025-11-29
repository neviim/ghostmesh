use warp::Filter;
use crate::state::AppState;
use tokio::sync::mpsc;
use std::net::SocketAddr;

pub async fn start_server(
    port: u16, 
    state: AppState, 
    log_tx: mpsc::UnboundedSender<String>
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
        .map(|bytes: bytes::Bytes, tx: mpsc::UnboundedSender<String>| {
            let msg = String::from_utf8_lossy(&bytes).to_string();
            if let Err(e) = tx.send(msg) {
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

    let routes = state_route
        .or(log_route)
        .or(index)
        .or(static_files);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Web Dashboard running at http://0.0.0.0:{}", port);

    warp::serve(routes).run(addr).await;
}
