use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};
use rustql_types::Database;
use rustql_types::TableFields;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppStore {
    pub databases: Vec<Database>,
    pub selected_db: Option<String>,
    pub selected_table: Option<String>,
    pub table_data: Option<TableFields>,
}

impl AppStore {
    pub fn new() -> Self {
        Self {
            databases: Vec::new(),
            selected_db: None,
            selected_table: None,
            table_data: None,
        }
    }
}