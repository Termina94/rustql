use yew::{prelude::*, Properties};

pub struct Collapse {
    link: ComponentLink<Self>,
    props: DropDownProps,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DropDownProps {
    #[prop_or_default]
    pub open: bool,
    #[prop_or_default]
    pub title: String,

    pub children: Children,
}

pub enum Msg {
    ToggleDraw,
}

impl Component for Collapse {
    type Message = Msg;

    type Properties = DropDownProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
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
            true => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="card mb-2">
                <div
                    class="card-header hand"
                    onclick=self.link.callback(|_| Msg::ToggleDraw)
                >
                    <p class="card-header-title noselect is-size-6 has-text-centered">
                        {&self.props.title}
                    </p>
                    <div class="card-header-icon hide-overflow">
                        <span class="icon">
                            <i class="fas fa-angle-down"/>
                        </span>
                    </div>
                </div>
                {self.view_draw()}
            </div>

        }
    }
}

impl Collapse {
    fn view_draw(&self) -> Html {
        match self.props.open {
            true => html! {
                <div class="panel-block">
                    <p class="control is-size-7 hide-overflow">
                        {self.props.children.clone()}
                    </p>
                </div>
            },
            false => html! {},
        }
    }
}
