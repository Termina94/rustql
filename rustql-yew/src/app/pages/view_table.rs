use std::{cell::RefCell, rc::Rc};
use rustql_types::{ApiAction, ApiRequest};
use yew::{Callback, Classes, Component, ComponentLink, DragEvent, Html, InputData, InputEvent, KeyboardEvent, MouseEvent, NodeRef, Properties, classes, html, services::{ConsoleService, websocket::WebSocketTask}, web_sys::HtmlElement};

use crate::app::{components::query_editor::{QueryEditor, QueryEditorMsg}, helpers::socket::Socket, store::AppStore, structs::page_view_link::CustomLink};

pub struct ViewTable {
    link: ComponentLink<Self>,
    editor_link: CustomLink<QueryEditor>,
    props: WelcomePageProps,
    query: String,
    query_box_open: bool,
    dragging: bool,
    query_box_height: i32,
    start_position: (i32, i32),
    splitter: NodeRef,

    // Event listeners

    drag: Callback<MouseEvent>,
    dragging_false: Callback<MouseEvent>,
    dragging_true: Callback<MouseEvent>,
    toggle_query_box_open: Callback<MouseEvent>,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct WelcomePageProps {
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
}

pub enum Msg {
    ToggleQueryBoxOpen,
    SetDragging(MouseEvent, bool),
    Drag(MouseEvent),
    AppendToQuery(String),
}

impl Component for ViewTable {
    type Message = Msg;
    type Properties = WelcomePageProps;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {

        let drag = link.callback(|drag| Msg::Drag(drag));
        let dragging_true = link.callback(|e| Msg::SetDragging(e, true));
        let dragging_false = link.callback(|e| Msg::SetDragging(e, false));
        let toggle_query_box_open = link.callback(|_| Msg::ToggleQueryBoxOpen);

        Self {
            link,
            editor_link: CustomLink::new(),
            props,
            query: String::new(),
            query_box_open: false,
            dragging: false,
            start_position: (0,0),
            query_box_height: 100,
            splitter: NodeRef::default(),
            // event listeners
            drag,
            dragging_false,
            dragging_true,
            toggle_query_box_open,
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::AppendToQuery(text) => {
                self.editor_link.send_message(QueryEditorMsg::AppendText(text.to_string(), None, None));
                false
            },
            Msg::ToggleQueryBoxOpen => { 
                self.query_box_open = !self.query_box_open;
                self.editor_link.send_message(QueryEditorMsg::Update(self.query_box_height));
                true
            },
            Msg::SetDragging(event, value) => {
                match value {
                    true => self.start_position = (event.screen_x(), event.screen_y()),
                    false => {
                        
                    }
                };
                self.dragging = value;
                false
            },
            Msg::Drag(event) => {
                if self.dragging {
                    self.query_box_height -= event.screen_y() - self.start_position.1;
                    self.start_position = (event.screen_x(), event.screen_y());

                    self.editor_link.send_message(QueryEditorMsg::Update(self.query_box_height));
                }
                false
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        match self.props == props {
            false => {
                self.props = props;
                true
            }
            true => true,
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <div 
                class="rows rows-fill"
                onmousemove=&self.drag
                onmouseup=&self.dragging_false
            >
                {self.view_rows()}
            </div>
                
        }
    }
}

impl ViewTable {
    fn view_rows(&self) -> Html {

        let db = self.props.store.borrow().get_db();
        let table = self.props.store.borrow().get_table();

        let append_db = self.link.callback(move |_| Msg::AppendToQuery(db.clone().unwrap()));
        let append_table = self.link.callback(move |_| Msg::AppendToQuery(table.clone().unwrap()));

        if true {
            html! {
                <>
                    <div class="row">
                        <div class="columns">
                            <div class="column">
                                <span class="icon-text">
                                    <span class="icon">
                                        <i class="fas fa-database"/>
                                    </span>
                                    <span onclick=append_db><b>{self.props.store.borrow().get_db().unwrap()}</b></span>
                                </span>
                                <br/>
                                <span class="icon-text">
                                    <span class="icon">
                                        <i class="fas fa-table"/>
                                    </span>
                                    <span onclick=append_table>{self.props.store.borrow().get_table().unwrap()}</span>
                                </span>
                            </div>
                            <div class="column">
                                {self.view_toolbar()}
                            </div>
                        </div>
                    </div>
                    <div class="row view-table mt-2 fill hide-overflow">
                        {self.view_table()}
                    </div>
                    {self.view_query_box()}
                </>
            }
        } else { Html::default() }
    }

    fn view_table(&self) -> Html {
        match &self.props.store.borrow().table_data {
            Some(data) => {
                let titles: Html = data
                    .table_fields
                    .iter()
                    .map(|field| {
                        html! { <th class="is-size-6">{&field.name }</th> }
                    })
                    .collect();

                let values = (0..data.count)
                    .map(|i| {
                        let rows = data
                            .table_fields
                            .iter()
                            .map(|field| {
                                let value: &String = field.values.get(i).unwrap();

                                html! { <td class="is-size-7">{value}</td> }
                            })
                            .collect::<Html>();

                        html! {
                            <tr>
                                {rows}
                            </tr>
                        }
                    })
                    .collect::<Html>();

                html! {
                    <div class="scrollable-all fill">
                        <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
                            <thead>
                                <tr>
                                    {titles}
                                </tr>
                            </thead>
                            <tbody>
                                {values}
                            </tbody>
                        </table>
                    </div>
                }
            }
            None => Html::default(),
        }
    }

    fn view_toolbar(&self) -> Html {
        html! {
            <div class="columns is-mobile">
                <div class="column is-narrow">
                    <button onclick=&self.toggle_query_box_open
                        class=classes!("button", self.query_box_open.then(||"is-info"))
                    >
                        <i class="is-medium fas fa-edit"/>
                    </button>
                </div>
                <div class="column is-narrow">
                    <button class="button">
                        <i class="is-medium fas fa-search"/>
                    </button>
                </div>
                <div class="column is-narrow">
                    <button class="button">
                        <i class="is-medium fas fa-table"/>
                    </button>
                </div>
            </div>
        }
    }

    fn view_query_box(&self) -> Html {
        let hide = !self.query_box_open;

        html! {
            <>
                <div class=classes!("row", "noselect", hide.then(||"no-display"))>
                    <div
                        class="split-dragger noselect"
                        ref=self.splitter.clone()
                        onmousedown=&self.dragging_true
                    />
                </div>
                <QueryEditor
                    hide=hide
                    store=self.props.store.clone()
                    height=self.query_box_height
                    editor_link=self.editor_link.clone()
                />
            </>
        }
    }
}
