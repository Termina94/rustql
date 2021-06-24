use rustql_types::{ApiAction, ApiRequest};
use yew::{Callback, Component, ComponentLink, Html, InputData, KeyboardEvent, NodeRef, Properties, classes, html::{onscroll::Event}, services::ConsoleService, web_sys::{HtmlElement, HtmlTextAreaElement}};
use std::{cell::RefCell, rc::Rc};
use yew::{html};

use crate::app::{helpers::functions::format_string_sql, store::AppStore, structs::page_view_link::CustomLink};

#[derive(Clone)]
pub struct QueryEditor {
    link: ComponentLink<QueryEditor>,
    update_query: Callback<InputData>,
    query_key_event: Callback<KeyboardEvent>,
    editor_scroll: Callback<Event>,
    input_ref: NodeRef,
    code_ref: NodeRef,
    props: QueryEditorProps,
    height: i32,
    text: String,
    code: String,
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
    Focus,
    ResetQuery,
    ClearQuery,
    EditorScroll,
    FormatDisplay,
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
        let update_query = link.callback( QueryEditorMsg::UpdateQuery);
        let query_key_event = link.callback(QueryEditorMsg::QueryKeyEvent);
        let editor_scroll = link.callback(|_| QueryEditorMsg::EditorScroll);

        *props.editor_link.link.borrow_mut() = Some(link.clone());

        QueryEditor {
            link,
            update_query,
            query_key_event,
            editor_scroll,
            input_ref: NodeRef::default(),
            code_ref: NodeRef::default(),
            height: props.height,
            props,
            text: String::new(),
            code: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            QueryEditorMsg::FormatDisplay => {
                if let Some(code_area) = self.code_ref.cast::<HtmlElement>() {
                    format_string_sql(&mut self.text);
                    code_area.set_inner_html(self.text.as_str());
                }
                true
            },
            QueryEditorMsg::EditorScroll => {
                if let Some(text_area) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    if let Some(code_area) = self.code_ref.cast::<HtmlElement>() {
                        ConsoleService::info(&format!("{}", text_area.scroll_top()));
                        code_area.set_scroll_top(text_area.scroll_top());
                    }
                }
                true
            },
            QueryEditorMsg::Focus => {
                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    text_box.set_autofocus(true);
                    text_box.focus().expect("Can't focus query box");
                    text_box.set_selection_start(Some(0)).unwrap();
                    text_box.set_selection_end(Some(0)).unwrap();
                }
                false
            },
            QueryEditorMsg::AppendText(text, start, end) => {

                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    
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
                    self.link.send_message(QueryEditorMsg::FormatDisplay);
                }
                true
            },
            QueryEditorMsg::ResetQuery => {
                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    self.text = self.props.store.try_borrow().unwrap().default_query();
                    text_box.set_value(self.text.as_str());
                }
                self.link.send_message(QueryEditorMsg::FormatDisplay);
                true
            },
            QueryEditorMsg::ClearQuery => {
                if let Some(text_box) = self.input_ref.cast::<HtmlTextAreaElement>() {
                    self.text = String::new();
                    text_box.set_value("");
                }
                self.link.send_message(QueryEditorMsg::FormatDisplay);
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
                self.link.send_message(QueryEditorMsg::FormatDisplay);
                false
            }
            QueryEditorMsg::QueryKeyEvent(event) => {
                // KeyboardEvent

                if event.alt_key() && event.shift_key() {
                    match event.key_code() {
                        13 => self.send_query(),
                        83 => self.append_shortcut(event, "SELECT * ", None, None),
                        70 => {
                            let text = &format!(
                                "FROM `{}`.`{}` ",
                                self.props.store.try_borrow().unwrap().selected_db.as_ref().unwrap(),
                                self.props.store.try_borrow().unwrap().selected_table.as_ref().unwrap(),
                            );
                            self.append_shortcut(event, text, None, None)
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
                            let start: Option<u32> = Some(text.chars().count() as u32 - 5);
                            let end: Option<u32> = Some(text.chars().count() as u32 - 1);
                            self.append_shortcut(event, text, start, end)
                        },
                        85 => self.append_shortcut(event, "UPDATE ", None, None),
                        87 => self.append_shortcut(event, "WHERE ", None, None),
                        68 => self.link.send_message(QueryEditorMsg::ResetQuery),
                        88 => self.link.send_message(QueryEditorMsg::ClearQuery),
                        _ => {}
                    }                    
                } else {
                    match event.key_code() {
                        9 => self.append_shortcut(event, "\t", None, None),
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
                        onscroll=&self.editor_scroll
                        autofocus=true
                    />
                    <code
                        class="textarea fill has-fixed-size query-code"
                        style="color: grey"
                        ref=self.code_ref.clone()
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
