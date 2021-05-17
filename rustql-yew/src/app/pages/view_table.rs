use rustql_types::TableField;
use std::{cell::RefCell, rc::Rc};
use yew::{html, services::ConsoleService, Component, ComponentLink, Html, Properties};

use crate::app::store::AppStore;

pub struct ViewTable {
    link: ComponentLink<Self>,
    props: WelcomePageProps,
}

#[derive(Clone, PartialEq, Properties, Debug)]
pub struct WelcomePageProps {
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
}

pub enum Msg {
    Debug,
}

impl Component for ViewTable {
    type Message = Msg;
    type Properties = WelcomePageProps;

    fn create(props: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        match msg {
            Msg::Debug => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> yew::ShouldRender {
        // Manually check until partialEq is implemented for store
        let table_change =
            self.props.store.borrow().selected_table == props.store.borrow().selected_table;
        let db_change = self.props.store.borrow().selected_db == props.store.borrow().selected_db;

        match self.props == props {
            false => {
                self.props = props;
                true
            }
            true => table_change || db_change,
        }
    }

    fn view(&self) -> yew::Html {
        if let (Some(db), Some(table)) = (
            &self.props.store.borrow().selected_db,
            &self.props.store.borrow().selected_table,
        ) {
            html! {
                <>
                    <div class="rows rows-fill">
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
                        <div class="row mt-2 fill hide-overflow">
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
            None => html! {},
        }
    }
}
