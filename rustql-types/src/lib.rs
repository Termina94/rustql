use std::{collections::HashMap};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, EnumString, Display)]
pub enum ApiAction {
    LoadTables,
    RunQuery,
    LoadTable,
    Init,
    Error,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Database {
    pub name: String,
    pub tables: Vec<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TableData {
    pub db_name: String,
    pub table_name: String,
    pub fields: TableFields,
}

pub type TableFields = HashMap<String, TableFieldData>;

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TableFieldData {
    pub field_type: String,
    pub values: Vec<String>,
}