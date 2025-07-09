use futures::{SinkExt, StreamExt}; // add .next() to read and .send() to write on streams and sinks
use log::*; // logging macros for runtime msgs
use std::env;
use warp::ws::Message; //Message enum defines types of messages sent/received
use warp::Filter;

#[tokio::main] // tokio-powered async runtime
async fn main() -> anyhow::Result<()> {
    // initalise logging output
    env_logger::init();

    // map / to serve frontend/index.html
    let index_html = warp::path::end().and(warp::fs::file("../frontend/index.html"));

    // read port number from Render
    let port: u16 = env::var("PORT")?.parse()?;

    // only match GET /ws requests and upgrade to WebSocket
    let ws_route = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|socket| async move {
            // split socket into sender and receiver
            let (mut tx, mut rx) = socket.split();

            // loop over each incoming msg
            while let Some(Ok(msg)) = rx.next().await {
                if msg.is_text() {
                    let reply = format!("echo: {}", msg.to_str().unwrap_or(""));
                    let _ = tx.send(Message::text(reply)).await;
                } else if msg.is_binary() {
                    let mut prefixed = b"echo: ".to_vec();
                    prefixed.extend_from_slice(&msg.as_bytes());
                    let _ = tx.send(Message::binary(prefixed)).await;
                } else if msg.is_close() {
                    break;
                }
            }
        })
    });

    // respond to Render health checks
    let healthz = warp::path("healthz").map(|| warp::reply());

    // static files from frontend folder
    let static_files = warp::fs::dir("../frontend");

    // combine routes and log each req
    let routes = ws_route
        .or(healthz)
        .or(index_html)
        .or(static_files)
        .with(warp::log("rust_ws_chat"));

    // start server on 0.0.0.0:PORT
    info!("Server running on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;

    Ok(())
}
