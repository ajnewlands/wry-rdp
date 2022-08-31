use anyhow::Result;
use wry::{
    application::{event_loop::EventLoop, window::WindowBuilder},
    webview::WebViewBuilder,
};

use wry::application::{
    event::{Event, StartCause, WindowEvent},
    event_loop::ControlFlow,
};

#[derive(Debug)]
pub enum CustomEvents {
    MakeVisible,
}

use log::*;

/// Implements the main window of the rdp user interface via wry webview
pub struct MainWindow {
    _webview: wry::webview::WebView,
    event_loop: wry::application::event_loop::EventLoop<CustomEvents>,
}

//const MAINSCRIPT: &str = include_str!("../js/main.js");
const MAINSCRIPT: &str = include_str!("../../typescript/out/app.js");
const MAINHTML: &str = include_str!("../html/main.html");

/// Create a webview which is initially invisible.
/// An ipc call from within the initial page view will trigger it to become visible.
/// This is to ensure the content has a chance to render properly before being displayed.
impl MainWindow {
    pub fn new(port_rx: tokio::sync::oneshot::Receiver<u16>) -> Result<Self> {
        let event_loop = EventLoop::<CustomEvents>::with_user_event();
        let window = WindowBuilder::new()
            .with_title("RDP-WRY")
            .with_visible(false)
            .build(&event_loop)?;
        let port = port_rx.blocking_recv().unwrap();
        info!("Websocket port will be {}", port);

        let proxy = event_loop.create_proxy();
        let event_handler =
            move |window: &wry::application::window::Window, req: String| match req.as_str() {
                "make-visible" => {
                    window.set_visible(true);
                    proxy.send_event(CustomEvents::MakeVisible).unwrap();
                }
                _ => info!("unhandled event {}", req),
            };

        let webview = WebViewBuilder::new(window)?
            .with_initialization_script(&format!("var WEBSOCKETADDRESS='127.0.0.1:{}';", port))
            .with_custom_protocol("app".into(), move |request| {
                info!("Request for URI: {}", request.uri());
                match request.uri().replace("app://", "").as_str() {
                    "internal/main.html" => wry::http::ResponseBuilder::new()
                        .mimetype("text/html")
                        .body(MAINHTML.as_bytes().to_vec()),
                    "internal/main.js" => wry::http::ResponseBuilder::new()
                        .mimetype("text/javascript")
                        .body(MAINSCRIPT.as_bytes().to_vec()),

                    _ => wry::http::ResponseBuilder::new()
                        .mimetype("text/html")
                        .body(Vec::default()),
                }
            })
            .with_ipc_handler(event_handler)
            .with_url("app://internal/main.html")?;

        #[cfg(debug_assertions)]
        let webview = webview.with_devtools(true);

        let webview = webview.build()?;

        #[cfg(debug_assertions)]
        webview.open_devtools();

        Ok(MainWindow {
            _webview: webview,
            event_loop,
        })
    }

    pub fn run_event_loop(self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => info!("Main window loaded."),
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
