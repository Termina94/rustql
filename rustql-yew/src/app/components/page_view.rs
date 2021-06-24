use crate::app::{pages::{view_table::{ViewTable}, welcome_page::WelcomePage}, store::AppStore, structs::page_view_link::CustomLink};
use std::{cell::RefCell, rc::Rc};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone)]
pub struct PageView {
    link: ComponentLink<Self>,
    props: PageViewProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageViewProps {
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
    pub page_link: CustomLink<PageView>,
}

pub enum PageViewMsg {
    Update,
}

impl Component for PageView {
    type Message = PageViewMsg;
    type Properties = PageViewProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        *props.page_link.link.borrow_mut() = Some(link.clone());

        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PageViewMsg::Update => true
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            false => {
                self.props = props;
                true
            }
            true => false,
        }
    }

    fn view(&self) -> Html {
        let db_selected = self.props.store.borrow().selected_db.is_some();
        let table_selected = self.props.store.borrow().selected_table.is_some();

        if table_selected && db_selected {
            html! {
                <ViewTable
                    store=self.props.store.clone()
                />
            }
        } else {
            html! {
                <WelcomePage />
            }
        }
    }
}
