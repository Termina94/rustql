use anyhow::Error;
use rustql_types::{ApiAction};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use yew::{
    format::Json,
    services::{
        websocket::{WebSocketStatus, WebSocketTask},
        WebSocketService,
    },
    Callback, Component, ComponentLink,
};

pub enum SocketMessage {
    Closed,
    Ignore,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    action: String,
    data: Option<String>,
}

pub trait Socket<T: Component> {
    fn on_message(msg: ApiAction, data: String) -> T::Message;

    fn on_notification(msg: SocketMessage) -> T::Message;

    fn create_socket(link: ComponentLink<T>) -> Option<WebSocketTask> {
        let on_message = Self::s_out(link.clone());
        let on_close = Self::s_close(link.clone());

        let task = WebSocketService::connect_text("ws://127.0.0.1:8888", on_message, on_close);
        match task {
            Ok(socket) => Some(socket),
            Err(_) => None,
        }
    }

    fn s_send(socket: &mut Option<WebSocketTask>, action: ApiAction) -> bool {
        match socket {
            Some(ref mut socket) => {
                socket.send(Ok(action.to_string()));
                true
            }
            None => false,
        }
    }

    fn s_out(link: ComponentLink<T>) -> Callback<Json<Result<ApiResponse, Error>>> {
        link.callback(|Json(msg): Json<Result<ApiResponse, Error>>| match msg {
            Ok(response) => match ApiAction::from_str(&response.action) {
                Ok(action) => Self::on_message(action, response.data.unwrap_or_default()),
                Err(_) => Self::on_message(ApiAction::Error, format!("{} not found", response.action)),
            },
            Err(err) => Self::on_message(ApiAction::Error, err.to_string()),
        })
    }

    fn s_close(link: ComponentLink<T>) -> Callback<WebSocketStatus> {
        link.callback(|status: WebSocketStatus| match status {
            WebSocketStatus::Closed | WebSocketStatus::Error => {
                Self::on_notification(SocketMessage::Closed)
            }
            _ => Self::on_notification(SocketMessage::Ignore),
        })
    }
}
