use regex::Regex;
use rustql_types::{ApiAction, ApiRequest};
use std::{borrow::Borrow, cell::RefCell, rc::Rc};
use yew::{Callback, Children, Component, ComponentLink, DragEvent, Html, InputData, InputEvent, KeyboardEvent, MouseEvent, NodeRef, Properties, classes, html, services::ConsoleService, web_sys::{self, HtmlElement, HtmlTextAreaElement}};

use crate::app::{store::AppStore, structs::page_view_link::CustomLink};

#[derive(Clone)]
pub struct QueryEditor {
    link: ComponentLink<QueryEditor>,
    update_query: Callback<InputData>,
    query_key_event: Callback<KeyboardEvent>,
    input_ref: NodeRef,
    props: QueryEditorProps,
    height: i32,
    text: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct QueryEditorProps {
    pub editor_link: CustomLink<QueryEditor>,
    pub height: i32,
    pub hide: bool,
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
}

pub enum QueryEditorMsg {
    ResetQuery,
    ClearQuery,
    AppendText(String, Option<u32>, Option<u32>),
    UpdateQuery(InputData),
    QueryKeyEvent(KeyboardEvent),
    Update(i32),
    Refresh,
}

impl Component for QueryEditor {
    type Message = QueryEditorMsg;
    type Properties = QueryEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let update_query = link.callback(|text| QueryEditorMsg::UpdateQuery(text));
        let query_key_event = link.callback(|event| QueryEditorMsg::QueryKeyEvent(event));

        *props.editor_link.link.borrow_mut() = Some(link.clone());

        QueryEditor {
            link,
            update_query,
            query_key_event,
            input_ref: NodeRef::default(),
            height: props.height,
            props,
            text: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            QueryEditorMsg::AppendText(text, start, end) => {

                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    
                    ConsoleService::info(&format!("{:?}", text_box.selection_start()));

                    let caret_position = text_box.selection_start().unwrap().unwrap();
                    let start_pos: u32 = match start {
                        Some(start) => caret_position + start as u32,
                        None => caret_position + text.chars().count() as u32
                    };
                    let end_pos: u32 = match end {
                        Some(end) => caret_position + end as u32,
                        None => caret_position + text.chars().count() as u32
                    };
        
                    self.text.insert_str(caret_position as usize, text.as_str());
                    text_box.set_value(self.text.as_str());
        
                    text_box.focus().unwrap();
                    text_box.set_selection_start(Some(start_pos)).unwrap();
                    text_box.set_selection_end(Some(end_pos)).unwrap();
                }
                true
            },
            QueryEditorMsg::ResetQuery => {
                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    self.text = self.props.store.try_borrow().unwrap().default_query();
                    text_box.set_value(self.text.as_str());
                }
                true
            },
            QueryEditorMsg::ClearQuery => {
                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    self.text = String::new();
                    text_box.set_value("");
                }
                true
            },
            QueryEditorMsg::UpdateQuery(text) => {
                self.text = text.value;

                // TODO create syntax highlighted text editor

                // let words = [
                //     "select",
                //     "from",
                //     "where",
                // ];

                // if let Some(input) = self.input_ref.cast::<HtmlTextAreaElement>() {

                //     let mut a = input.value();

                //     words.iter().for_each(|word| {

                //         let re = Regex::new(&format!("(?:^|\\s|[^a-z])({})(^|\\s|[^a-z])", word)).unwrap();

                //         a = re.replace_all(&a, word.to_uppercase()).to_string();
                //     });

                //     input.set_value(a.as_str());
                // }

                false
            }
            QueryEditorMsg::QueryKeyEvent(event) => {
                // KeyboardEvent

                ConsoleService::info(&format!("{}", event.key_code()));

                if event.alt_key() && event.shift_key() {
                    match event.key_code() {
                        13 => self.send_query(),
                        83 => self.append_shortcut(event, "Select * ", None, None),
                        70 => {
                            let text = &format!(
                                "FROM `{}`.`{}`",
                                self.props.store.try_borrow().unwrap().selected_db.as_ref().unwrap(),
                                self.props.store.try_borrow().unwrap().selected_table.as_ref().unwrap(),
                            );
                            let end: Option<u32> = Some(text.chars().count() as u32 - 2);
                            self.append_shortcut(event, text, Some(6), end)
                        },
                        73 => {
                            let text = &format!(
                                "INSERT INTO `{}`.`{}` (``)\nVALUES('')",
                                self.props.store.try_borrow().unwrap().selected_db.as_ref().unwrap(),
                                self.props.store.try_borrow().unwrap().selected_table.as_ref().unwrap(),
                            );
                            let pos: Option<u32> = Some(text.chars().count() as u32 - 13);
                            self.append_shortcut(event, text, pos, pos)
                        },
                        76 => self.append_shortcut(event, "LIMIT 24", Some(6), Some(8)),
                        74 => {
                            let text = &format!(
                                "LEFT JOIN `{}`.`{}` as `name`",
                                self.props.store.try_borrow().unwrap().selected_db.as_ref().unwrap(),
                                self.props.store.try_borrow().unwrap().selected_table.as_ref().unwrap(),
                            );
                            let start: Option<u32> = Some(text.chars().count() as u32 - 6);
                            let end: Option<u32> = Some(text.chars().count() as u32 - 2);
                            self.append_shortcut(event, text, start, end)
                        },
                        85 => self.append_shortcut(event, "UPDATE ", None, None),
                        87 => self.append_shortcut(event, "WHERE ", None, None),
                        68 => self.link.send_message(QueryEditorMsg::ResetQuery),
                        88 => self.link.send_message(QueryEditorMsg::ClearQuery),
                        _ => {}
                    }                    
                }
                false
            }
            QueryEditorMsg::Refresh => true,
            QueryEditorMsg::Update(height) => {
                self.height = height;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        match self.props == props {
            false => {
                self.props = props;
                true
            }
            true => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div
                    class=classes!(
                        "query-box",
                        "row",
                        "mt-2",
                        "hide-overflow",
                        self.props.hide.then(||"no-display")
                    )
                    style=format!("height: {}px", self.height)
                >
                    <textarea
                        ref=self.input_ref.clone()
                        contenteditable="true"
                        class="textarea fill has-fixed-size query-text"
                        placeholder="Ctrl + Enter to run query..."
                        oninput=&self.update_query
                        onkeydown=&self.query_key_event
                    />
                </div>
            </>
        }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

impl QueryEditor {
    pub fn send_query(&self) {
        let data: (String, String, String) = (
            self.props
                .store
                .try_borrow()
                .expect("Could not borrow store")
                .selected_db
                .as_ref()
                .expect("QueryKeyEvent expects db, failed")
                .clone(),
            self.props
                .store
                .try_borrow()
                .expect("Could not borrow store")
                .selected_table
                .as_ref()
                .expect("QueryKeyEvent expects table, failed")
                .clone(),
            self.text.clone(),
        );

        self.props
            .store
            .try_borrow()
            .expect("Could not borrow store")
            .socket_send(ApiRequest::create_data(ApiAction::RunQuery, data));
    }

    pub fn append_shortcut(&mut self, event: KeyboardEvent, text: &str, start: Option<u32>, end: Option<u32>) {
        event.prevent_default();
        self.link.send_message(QueryEditorMsg::AppendText(String::from(text), start, end));
    }
}