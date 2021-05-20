use std::{cell::RefCell, rc::Rc};
use regex::Regex;
use rustql_types::{ApiAction, ApiRequest};
use yew::{Callback, Children, Component, ComponentLink, DragEvent, Html, InputData, InputEvent, KeyboardEvent, MouseEvent, NodeRef, Properties, classes, html, services::ConsoleService, web_sys::{HtmlElement, HtmlTextAreaElement}};

use crate::app::{store::AppStore, structs::page_view_link::CustomLink};

#[derive(Clone, PartialEq)]
pub struct QueryEditor {
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
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
}

pub enum QueryEditorMsg {
    UpdateQuery(InputData),
    QueryKeyEvent(KeyboardEvent),
    Update(i32),
    Refresh
}

impl Component for QueryEditor {
    type Message = QueryEditorMsg;
    type Properties = QueryEditorProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let update_query = link.callback(|text| QueryEditorMsg::UpdateQuery(text));
        let query_key_event = link.callback(|event| QueryEditorMsg::QueryKeyEvent(event));

        *props.editor_link.link.borrow_mut() = Some(link.clone());

        QueryEditor{
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
            },
            QueryEditorMsg::QueryKeyEvent(event) => {
                // KeyboardEvent
                if event.ctrl_key() && event.key_code() == 13 {

                    let data: (String, String, String) = (
                        self.props.store.borrow().selected_db.as_ref().unwrap().clone(),
                        self.props.store.borrow().selected_table.as_ref().unwrap().clone(),
                        self.text.clone()
                    );

                    self.props.store.borrow().socket_send(
                        ApiRequest::create_data(ApiAction::RunQuery, data)
                    );
                }
                false
            },
            QueryEditorMsg::Refresh => true,
            QueryEditorMsg::Update(height) => {
                self.height = height;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="query-box row mt-2 hide-overflow" style=format!("height: {}px", self.height)>
                <textarea
                    ref=self.input_ref.clone()
                    contenteditable="true"
                    class="textarea fill has-fixed-size query-text"
                    placeholder="Ctrl + Enter to run query..."
                    oninput=&self.update_query
                    onkeydown=&self.query_key_event
                />
            </div>
        }
    }
}