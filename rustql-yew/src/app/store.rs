use rustql_types::{ApiRequest};
use rustql_types::{Database, TableData};
use yew::{Callback, Component, ComponentLink};
use crate::app::Msg;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppStore {
    pub socket_link: Callback<ApiRequest>,
    pub databases: Vec<Database>,
    pub selected_db: Option<String>,
    pub selected_table: Option<String>,
    pub table_data: Option<TableData>,
}

impl AppStore {
    pub fn new() -> Self {
        Self {
            socket_link: Callback::default(),
            databases: Vec::new(),
            selected_db: None,
            selected_table: None,
            table_data: None,
        }
    }

    pub fn set_socket_link<T:Component>(&mut self, link: ComponentLink<T>)
    where <T as yew::Component>::Message: From<Msg>
    {
        self.socket_link = link.callback(|request| Msg::SocketSend(request))
    }

    pub fn socket_send(&self, msg: ApiRequest) {
        self.socket_link.emit(msg);
    }
}
