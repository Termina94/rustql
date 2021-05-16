use crate::app::store::AppStore;

use super::collapse::Collapse;
use rustql_types::{ApiAction, Database};
use std::{cell::RefCell, rc::Rc, usize};
use yew::{prelude::*, virtual_dom::VNode, Properties};

pub struct DBCollapse {
    link: ComponentLink<Self>,
    props: DBCollapseProps,
    search_field: String,
}

pub enum DBCollapseMsg {
    UpdateSearch(InputData),
    ClearSearch,
}

#[derive(Clone, PartialEq, Properties)]
pub struct DBCollapseProps {
    #[prop_or_default]
    pub store: Rc<RefCell<AppStore>>,
    pub socket: Callback<ApiAction>,
    pub on_selected: Callback<(usize, usize)>,
}

impl Component for DBCollapse {
    type Message = DBCollapseMsg;
    type Properties = DBCollapseProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            search_field: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DBCollapseMsg::UpdateSearch(search) => {
                self.search_field = search.value;
                true
            }
            DBCollapseMsg::ClearSearch => {
                self.search_field = String::new();
                true
            }
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
        html! {
            <>
                <div class="field has-addons mb-2">
                    <p class="control has-icons-left is-expanded">
                        <input
                            class="input is-primary" type="text"
                            placeholder="Search"
                            value=self.search_field
                            oninput=self.link.callback(|search| DBCollapseMsg::UpdateSearch(search))
                        />
                        <span class="icon is-left">
                            <i class="fas fa-search" aria-hidden="true"/>
                        </span>
                    </p>
                    <div class="control">
                        <a
                            class="button is-info"
                            onclick=self.link.callback(|_| DBCollapseMsg::ClearSearch)
                        >
                            <span class="icon hand">
                                <i class="fas fa-times"/>
                            </span>
                        </a>
                    </div>
                </div>
                {self.view_database_list()}
            </>
        }
    }
}

impl DBCollapse {
    fn view_database_list(&self) -> Html {
        match &self.search_field == "" {
            true => self.view_all_database_list(),
            false => self.view_search_list(),
        }
    }

    fn view_all_database_list(&self) -> Html {
        self.props
            .store
            .borrow()
            .databases
            .iter()
            .enumerate()
            .map(|(db_id, item)| {
                html! {
                    <Collapse title=item.name.clone()>
                        { for item.tables.iter().enumerate().map(|(i, table)| {
                            self.view_table_selector(table, i, db_id)
                        })}
                    </Collapse>
                }
            })
            .collect()
    }

    fn view_search_list(&self) -> Html {
        let dbs = self
            .props
            .store
            .borrow()
            .databases
            .iter()
            .enumerate()
            .map(|(db_id, db)| {
                let tables: Vec<(usize, &String)> = db
                    .tables
                    .iter()
                    .enumerate()
                    .filter(|(_, table)| {
                        table
                            .to_lowercase()
                            .contains(&self.search_field.to_lowercase())
                    })
                    .collect();

                if tables.len() > 0 {
                    html! {
                        <Collapse open=true title=&db.name>
                            { for tables.iter().map(|(table_id, table)| {
                                self.view_table_selector(table, *table_id, db_id)
                            })}
                        </Collapse>
                    }
                } else {
                    Html::default()
                }
            })
            .collect();

        dbs
    }

    fn view_table_selector(&self, table_name: &String, table_id: usize, db_id: usize) -> VNode {
        html! {
            <a
                class="panel-block"
                onclick=self.props.on_selected.reform(move|_| (table_id, db_id))
            >
                {table_name}
            </a>
        }
    }
}
