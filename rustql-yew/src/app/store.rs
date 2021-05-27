use std::{borrow::Borrow, cell::{Ref, RefCell}, rc::Rc};

use crate::app::Msg;
use rustql_types::ApiRequest;
use rustql_types::{Database, TableData};
use yew::{Callback, Component, ComponentLink};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppStore {
    pub socket_link: Callback<ApiRequest>,
    pub databases: Vec<Database>,
    pub selected_db: Option<String>,
    pub selected_table: Option<String>,
    pub table_data: Option<TableData>,
    pub default_limit: i32,
}

impl AppStore {
    pub fn new() -> Self {
        Self {
            socket_link: Callback::default(),
            databases: Vec::new(),
            selected_db: None,
            selected_table: None,
            table_data: None,
            default_limit: 24
        }
    }

    pub fn get_db(&self) -> Option<String> {
        self.selected_db.clone()
    }

    pub fn get_table(&self) -> Option<String> {
        self.selected_table.clone()
    }

    pub fn default_query(&self) -> String {
        format!("SELECT * FROM {}.{} \nLIMIT {}", 
            self.selected_db.as_ref().unwrap(),
            self.selected_table.as_ref().unwrap(),
            self.default_limit
        )
    }

    pub fn set_socket_link<T: Component>(&mut self, link: ComponentLink<T>)
    where
        <T as yew::Component>::Message: From<Msg>,
    {
        self.socket_link = link.callback(|request| Msg::SocketSend(request))
    }

    pub fn socket_send(&self, msg: ApiRequest) {
        self.socket_link.emit(msg);
    }
}
