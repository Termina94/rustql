use log::debug;
use mysql::{Error, Pool, Row, prelude::{Queryable}};
use rustql_types::{ApiAction, Database, TableFields};
use serde::{Deserialize, Serialize};
use std::{str::FromStr};
use crate::helpers::api_types::{self, table_fields_from};

pub async fn run_action(action: String) -> String {
    let response = match ApiAction::from_str(&action) {
        Ok(num) => match num {
            ApiAction::LoadTables => load_tables().await,
            ApiAction::RunQuery => run_query().await,
            ApiAction::LoadTable => load_table().await,
            _ => Ok(send_error(String::from("ApiAction Not Implemented"))),
        },
        Err(_) => Ok(send_error(format!("ApiAction not found: {}", action))),
    };

    match response {
        Ok(result) => result,
        Err(err) => send_error(err.to_string()),
    }
}

pub fn send_error(err: String) -> String {
    let error = ApiResponse {
        action: ApiAction::Error.to_string(),
        data: Some(err.to_string()),
    };

    serde_json::to_string(&error).unwrap_or_default()
}

pub fn send_event(action: ApiAction) -> String {
    send_json(action, "")
}

pub fn send_json<T: Serialize>(action: ApiAction, data: T) -> String {
    let json = serde_json::to_string(&data);

    debug!("{:?}", &json);

    let reponse_object: ApiResponse = match json {
        Ok(res) => ApiResponse {
            action: action.to_string(),
            data: Some(res),
            ..Default::default()
        },
        Err(err) => ApiResponse {
            action: ApiAction::Error.to_string(),
            data: Some(err.to_string()),
            ..Default::default()
        },
    };

    match serde_json::to_string(&reponse_object) {
        Ok(res) => res,
        Err(err) => send_error(err.to_string()),
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    action: String,
    data: Option<String>,
}

impl Default for ApiResponse {
    fn default() -> Self {
        Self {
            action: String::default(),
            data: None,
        }
    }
}

pub async fn load_tables() -> Result<String, Error> {
    let url = "mysql://root:rustqlpw@localhost:3306";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let db_names: Vec<String> = conn.query("SHOW DATABASES")?;
    let databases = db_names
        .iter()
        .map(|name| {
            let tables: Vec<String> = conn
                .query(format!("SHOW TABLES FROM {}", name))
                .unwrap_or_default();
            Database {
                name: name.to_owned(),
                tables,
            }
        })
        .collect();

    Ok(send_json::<Vec<Database>>(ApiAction::LoadTables, databases))
}

pub async fn load_table() -> Result<String, Error> {
    let url = "mysql://root:rustqlpw@localhost:3306";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    let results: Vec<Row> = conn.query("SELECT * FROM sys.host_summary")?;
    


    Ok(send_json::<TableFields>(ApiAction::LoadTable, table_fields_from(results)))
}

pub async fn run_query() -> Result<String, Error> {
    Ok(send_json(ApiAction::RunQuery, "Test"))
}
