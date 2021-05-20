use self::{components::page_view::PageViewMsg, store::AppStore};
use components::page_view::PageView;
use components::{db_collapse::DBCollapse, navbar::Navbar};
use helpers::socket::Socket;
use helpers::socket::SocketMessage;
use rustql_types::{ApiAction, ApiRequest, Database, TableData};
use serde_json;
use std::{cell::RefCell, rc::Rc};
use structs::page_view_link::CustomLink;
use yew::prelude::*;
use yew::{
    services::{websocket::WebSocketTask},
};

mod components;
mod helpers;
mod pages;
mod store;
mod structs;

pub struct App {
    link: ComponentLink<Self>,
    socket: Option<WebSocketTask>,
    state: State,
    store: Rc<RefCell<AppStore>>,
    page_link: CustomLink<PageView>,
}

pub enum State {
    Loaded,
    Loading,
    Errored { error: String },
    Closed { message: String },
}

pub enum Msg {
    LoadDatabases(Result<Vec<Database>, serde_json::Error>),
    UpdateTableData(TableData),
    Ignore,
    SocketInit,
    ResetSocket,
    SocketClosed,
    SocketSend(ApiRequest),
    SocketError(String),
    TableSelected((usize, usize)),
}

impl Socket<App> for App {
    fn on_message(action: ApiAction, data: String) -> Msg {
        match action {
            ApiAction::LoadTables => Msg::LoadDatabases(serde_json::from_str(&data)),
            ApiAction::Error => Msg::SocketError(data),
            ApiAction::Init => Msg::SocketInit,
            ApiAction::LoadTable => match serde_json::from_str(&data) {
                Ok(value) => Msg::UpdateTableData(value),
                Err(err) => Msg::SocketError(err.to_string()),
            },
            _ => Msg::Ignore,
        }
    }

    fn on_notification(message: SocketMessage) -> Msg {
        match message {
            SocketMessage::Closed => Msg::SocketClosed,
            _ => Msg::Ignore,
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        let store = Rc::new(RefCell::new(AppStore::new()));
        store.borrow_mut().set_socket_link(link.clone());

        App {
            link: link.clone(),
            state: State::Loaded,
            socket: Self::create_socket(link.clone()),
            store,
            page_link: CustomLink::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SocketInit => {
                Self::s_send(&mut self.socket, ApiRequest::create(ApiAction::LoadTables));
                self.state = State::Loaded;
                true
            }
            Msg::SocketSend(request) => {
                Self::s_send(&mut self.socket, request);
                false
            }
            Msg::SocketClosed => {
                self.state = State::Closed {
                    message: String::from("Socket Closed"),
                };
                true
            }
            Msg::SocketError(error) => {
                self.state = State::Errored { error };
                true
            }
            Msg::ResetSocket => {
                self.socket = {
                    let new_socket = Self::create_socket(self.link.clone());
                    self.state = match new_socket {
                        Some(_) => State::Loading,
                        None => State::Errored {
                            error: String::from("Socket Failed to reset"),
                        },
                    };
                    new_socket
                };
                true
            }
            Msg::LoadDatabases(Ok(dbs)) => {
                self.store
                    .try_borrow_mut()
                    .expect("Can't Borrow Store (Msg::LoadDatabases)")
                    .databases = dbs;

                true
            }
            Msg::TableSelected((table_id, db_id)) => {
                self.store
                    .try_borrow_mut()
                    .expect("Can't Borrow Store (Msg::TableSelected)")
                    .selected_db = Some(self.get_db(db_id));
                self.store
                    .try_borrow_mut()
                    .expect("Can't Borrow Store (Msg::TableSelected)")
                    .selected_table = Some(self.get_table(db_id, table_id));
                self.link
                    .send_message(Msg::SocketSend(ApiRequest::create_data(
                        ApiAction::LoadTable,
                        (
                            self.store.borrow().selected_db.as_ref().unwrap(),
                            self.store.borrow().selected_table.as_ref().unwrap(),
                        ),
                    )));
                false
            }
            Msg::UpdateTableData(fields) => {
                self.store
                    .try_borrow_mut()
                    .expect("Can't Borrow Store (Msg::UpdateTableData)")
                    .table_data = Some(fields);

                // update page only on successful query
                self.page_link
                    .link
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send_message(PageViewMsg::Update);
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                {self.warning_banner().unwrap_or_default()}
                <Navbar/>
                <div class="columns m-2 fill hide-overflow">

                    <div class="db-collapse column p-1 fill scrollable">
                        <DBCollapse
                            store=self.store.clone()
                            on_selected=self.link.callback(Msg::TableSelected)
                        />
                    </div>

                    <div class="column p-1 fill hide-overflow">
                        <div class="box fill hide-overflow">
                            {self.view_page()}
                        </div>
                    </div>

                </div>
            </div>
        }
    }
}

impl App {
    pub fn warning_banner(&self) -> Option<Html> {
        match &self.state {
            State::Loading => {
                Some(self.warning_modal("Websocket Connecting", String::from("Loading..."),
            html! {}))
            }
            State::Errored { error } => {
                Some(self.warning_modal("Error Occured", format!("Details: {}", error),
            html! {
                        <button onclick=self.link.callback(|_| Msg::ResetSocket) class="button is-success">{"Okay"}</button>
                    }))
            }
            State::Closed { message } => {
                Some(self.warning_modal("Websocket Closed", format!("Details: {}", message),
                html! {
                        <button onclick=self.link.callback(|_| Msg::ResetSocket) class="button is-success">{"Try Reconnect"}</button>
                    }))
            }
            _ => None,
        }
    }

    pub fn warning_modal(&self, title: &str, body: String, button: Html) -> Html {
        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-card">
                    <div class="modal-card-head">
                        <div class="modal-card-title">
                            {title}
                        </div>
                    </div>
                    <div class="modal-card-body">
                        {body}
                    </div>
                    <div class="modal-card-foot">
                        {button}
                    </div>
                </div>
            </div>
        }
    }

    pub fn view_page(&self) -> Html {
        html! {
            <PageView
                store=self.store.clone()
                page_link=self.page_link.clone()>
            </PageView>
        }
    }
}
