use rustql_types::TableFields;
use rustql_types::{Database, TableData};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct AppStore {
    pub databases: Vec<Database>,
    pub selected_db: Option<String>,
    pub selected_table: Option<String>,
    pub table_data: Option<TableData>,
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
