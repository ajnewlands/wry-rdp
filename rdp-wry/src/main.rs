// TODO get these from the front end

fn main() {
    env_logger::init();

    // Used to pass the ephemeral port number from the websocket server
    // to the javascript interpreter in the webview to enable a connection to be made.
    let (port_tx, port_rx) = tokio::sync::oneshot::channel::<u16>();
    let runtime = std::thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        rt.block_on(async move {
            let _ = ws::websocket_listen(port_tx).await;
        });
    });

    let main_window = wv::MainWindow::new(port_rx).unwrap();
    main_window.run_event_loop();

    let _ = runtime.join();
}
