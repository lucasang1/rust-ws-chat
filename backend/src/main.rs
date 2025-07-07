use futures::{SinkExt, StreamExt}; // add .next() to read and .send() to write on streams and sinks
use log::*; // logging macros for runtime msgs
use tokio::net::{TcpListener, TcpStream}; //Listener: accept new cxns, Stream: a cxn
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message; //Message enum defines types of messages sent/received // async upgrades raw TcpStream into WebSocket cxn

#[tokio::main] // tokio-powered async runtime
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init(); // print logging macros
    let addr = "0.0.0.0:9001";
    let listener = TcpListener::bind(addr).await?; // bind listeners to port 9001, wait till binding completes or error
    info!("WebSocket server listening on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        // wait for a new cxn. on success, return (TcpStream, SocketAddr)
        tokio::spawn(handle_connection(stream));
        // spawn new async task to handle this in parallel. so can handle others simul.
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream) {
    let peer = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("New connection from {}", peer);

    let ws_stream = match accept_async(stream).await {
        // attempt websocket handshake over raw stream
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    // split stream into writer and reader using stream/sink traits

    while let Some(msg) = read.next().await {
        // fetch next msg
        match msg {
            Ok(Message::Text(text)) => {
                // extract string from msg
                info!("Received from {}: {}", peer, text);
                if let Err(e) = write.send(Message::Text(text)).await {
                    error!("Error sending echo: {}", e);
                    break;
                }
            }
            Ok(Message::Close(_)) => {
                // client closes
                info!("{} disconnected", peer);
                break;
            }
            _ => {}
        }
    }
    info!("Connection {} closed.", peer);
}
