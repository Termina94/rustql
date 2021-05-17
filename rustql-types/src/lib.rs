use serde::{Deserialize, Serialize};
use std::{collections::HashMap, usize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, EnumString, Display)]
pub enum ApiAction {
    LoadTables,
    RunQuery,
    LoadTable,
    Init,
    Error,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ApiRequest {
    pub action: String,
    pub data: Option<String>,
}

impl ApiRequest {
    pub fn create_data<T: Serialize>(action: ApiAction, data: T) -> ApiRequest {
        let request_data = match serde_json::to_string(&data) {
            Ok(d) => Some(d),
            Err(_) => None,
        };

        ApiRequest {
            action: action.to_string(),
            data: request_data,
        }
    }

    pub fn create(action: ApiAction) -> ApiRequest {
        ApiRequest {
            action: action.to_string(),
            data: None,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ApiResponse {
    pub action: String,
    pub data: Option<String>,
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
    pub table_fields: TableFields,
    pub count: usize,
}

pub type TableFields = Vec<TableField>;

#[derive(Default, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct TableField {
    pub name: String,
    pub sql_type: String,
    pub values: Vec<String>,
}

pub enum TableTypes {
    TableNULL,
    TableBytes,
    TableInt,
    TableUInt,
    TableFloat,
    TableDouble,
    TableDate,
    TableTime,
}
