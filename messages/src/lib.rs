use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum FromBrowserMessages {
    RDPConnect(RDPConfiguration),
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
}

#[derive(Deserialize, Debug)]
pub struct RDPConfiguration {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct MouseEvent {
    pub action: String,
    pub button: String,
    pub x: i32, // Just in case the browser ever gives us a negative offset.
    pub y: i32,
}

#[derive(Deserialize, Debug)]
pub struct KeyboardEvent {
    pub action: String,
    pub key: String,
}
