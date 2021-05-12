use yew::{prelude::*, Properties};

pub struct Navbar {
    link: ComponentLink<Self>,
    props: DropDownProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DropDownProps {
    #[prop_or_default]
    pub open: bool
}

pub enum Msg {
    ToggleDraw
}

impl Component for Navbar {
    type Message = Msg;

    type Properties = DropDownProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleDraw => self.props.open = !self.props.open,
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        match self.props == props {
            false => {
                self.props = props;
                true
            }
            true => false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-menu">
                    <div class="navbar-start">
                        <a href="/" class="navbar-item">
                            {String::from("Home")}
                        </a>
                    </div>
                </div>                
            </div>
        }
    }
}