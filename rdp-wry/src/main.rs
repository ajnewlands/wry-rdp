// TODO get these from the front end

fn main() {
    let username: &str = &std::env::var("RDPUSER").unwrap();
    let password: &str = &std::env::var("RDPPASSWORD").unwrap();
    let host: &str = &std::env::var("RDPHOST").unwrap();
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

    // TODO need a selector which has access both to the websocket and to events
    // coming out of the RDP instance in order to convey events RDP <=> Webview
    /*
    let rdp_configuration =
        rdp::RDPConfiguration::new(host.into(), 3389, username.into(), password.into());
    let rdp_thread = std::thread::spawn(move || {
        let rdp = rdp::RDP::new(rdp_configuration).unwrap();
        rdp.start().unwrap();
    });
    */

    let main_window = wv::MainWindow::new(port_rx).unwrap();
    main_window.run_event_loop();

    let _ = runtime.join();
    //let _ = rdp_thread.join();
}
