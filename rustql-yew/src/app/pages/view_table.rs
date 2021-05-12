use std::{cell::RefCell, rc::Rc};
use yew::{Component, ComponentLink, Html, Properties, html, services::ConsoleService};

use crate::app::store::AppStore;

pub struct ViewTable {
    link: ComponentLink<Self>,
    props: WelcomePageProps
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct WelcomePageProps {
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
}

pub enum Msg {
    Debug
}

impl Component for ViewTable {
    type Message = Msg;
    type Properties = WelcomePageProps;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
           
            Msg::Debug => {
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {

        // Manually check until partialEq is implemented for store
        let table_change = self.props.store.borrow().selected_table == props.store.borrow().selected_table;
        let db_change = self.props.store.borrow().selected_db == props.store.borrow().selected_db;

        match self.props == props {
            false => {
                self.props = props;
                true
            }
            true => table_change || db_change
        }
    }

    fn view(&self) -> yew::Html {
        if let (Some(db), Some(table) ) = 
        ( &self.props.store.borrow().selected_db, &self.props.store.borrow().selected_table) {
            html! {
                <>
                    <div class="rows">
                        <div class="row">
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="fas fa-database"/>
                                </span>
                                <span><b>{db}</b></span>
                            </span>
                            <br/>
                            <span class="icon-text">
                                <span class="icon">
                                    <i class="fas fa-table"/>
                                </span>
                                <span>{table}</span>
                            </span>
                        </div>
                        <div class="row">
                            {self.view_table()}
                        </div>
                    </div>
                </>
            }
        } else {
            html! {
                <>
                </>
            }
        }
    }
}

impl ViewTable {
    fn view_table(&self) -> Html {

        match &self.props.store.borrow().table_data {
            Some(data) => {
                data.iter().map(|(key, value)| {
                    html!{
                        <>
                            {key}
                            <br/>
                        </>
                    }
                }).collect()
            },
            None => html!{}
        }
    }
}