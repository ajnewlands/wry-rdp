use anyhow::Result;
use futures_util::{future, SinkExt, StreamExt, TryStreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

use messages::FromBrowserMessages;

pub async fn websocket_listen(port_tx: tokio::sync::oneshot::Sender<u16>) -> Result<()> {
    let addr = "127.0.0.1:0";

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", listener.local_addr().unwrap());
    port_tx.send(listener.local_addr().unwrap().port()).unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();

    let (screen_tx, mut screen_rx) = mpsc::unbounded_channel::<Vec<u8>>();
    rdp::SCREEN_TX
        .set(screen_tx)
        .expect("rdp::SCREEN_TX already initialized");

    let mut o_command_tx = None;

    loop {
        tokio::select! {
            delivery = read.next() => match delivery {
             Some(Ok(msg)) => {
                if msg.is_text() {
                    match serde_json::from_slice::<FromBrowserMessages>(&msg.into_data()).unwrap() {
                        FromBrowserMessages::RDPConnect(cfg) => {
                            let (command_tx, command_rx) = mpsc::unbounded_channel::<FromBrowserMessages>();
                            o_command_tx = Some(command_tx);
                            let _rdp_thread = std::thread::spawn(move || {
                                let rdp = rdp::RDP::new(cfg).unwrap();
                                rdp.start(command_rx).unwrap();
                            });
                        }
                        msg => {
                            if let Some(tx) = &o_command_tx {
                                tx.send(msg).unwrap();
                            }
                        }
                    }
                } else if msg.is_binary() {
                    log::error!("Received unexpected binary payload");
                    break;
                }
            }
            Some(Err(e)) => {
                log::error!("Websocket stream ended: {:?}", e);
                break;
            }
            None => {
                log::info!("Websocket closed");
                break;
            }

            },
            rx = screen_rx.recv() =>  {
                if let Some(data) = rx {
                    write.send(tokio_tungstenite::tungstenite::Message::Binary(data)).await.expect(
                     "Failed to send screen via websocket"
                    );

                }
            }
        }
    }
}
