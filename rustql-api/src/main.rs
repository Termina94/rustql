use controllers::mysql::{self, send_event, send_json};
use log::debug;
use rustql_types::{ApiAction, ApiRequest};
use std::net::TcpListener;
use tungstenite::Message;

mod controllers;
mod helpers;

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    create_websocket_listener();
}

fn create_websocket_listener() {
    let server = TcpListener::bind("127.0.0.1:8888").unwrap();
    for stream in server.incoming() {
        tokio::spawn(async {
            let callback = |_req: &Request, response: Response| Ok(response);
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            // initial message to react on socket reconnect
            websocket
                .write_message(Message::Text(send_event(ApiAction::Init)))
                .unwrap();

            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_text() && !msg.is_empty() {
                    match serde_json::from_str::<ApiRequest>(&msg.to_string()) {
                        Ok(request) => {
                            let response = mysql::run_action(request);

                            websocket
                                .write_message(Message::Text(response.await))
                                .unwrap();
                        }
                        Err(err) => debug!("{}", err.to_string()),
                    }
                }
            }
        });
    }
}
