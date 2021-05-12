use std::{cell::RefCell, rc::Rc};

use yew::{Properties, Component, ComponentLink, html, services::ConsoleService};


pub struct WelcomePage {
    link: ComponentLink<Self>,
    props: WelcomePageProps
}

#[derive(Clone, PartialEq, Properties)]
pub struct WelcomePageProps {
}

pub enum Msg {
    Debug
}

impl Component for WelcomePage {
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
        match self.props == props {
            true => false,
            false => {
                self.props = props;
                true
            }
        }
    }

    fn view(&self) -> yew::Html {
        html! {
            <div>
                <h1>{String::from("Welcome to RustQl")}</h1>
            </div>
        }
    }
}