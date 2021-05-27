use std::str::FromStr;

use crate::helpers::api_types::table_fields_from;
use log::debug;
use mysql::{prelude::Queryable, Error, Pool};
use rustql_types::{ApiAction, ApiRequest, ApiResponse, Database, TableData};
use serde::Serialize;

// static URL: &str = "mysql://node_user:ydKlroqpPUnAKSjA@192.168.1.105:3306";
static URL: &str = "mysql://root:rustqlpw@localhost:3306";

pub async fn run_action(request: ApiRequest) -> String {
    let response = match ApiAction::from_str(&request.action) {
        Ok(num) => match num {
            ApiAction::LoadTables => load_tables().await,
            ApiAction::RunQuery => run_query(request).await,
            ApiAction::LoadTable => load_table(request).await,
            _ => Ok(send_error(String::from("ApiAction Not Implemented"))),
        },
        Err(_) => Ok(send_error(format!(
            "ApiAction not found: {}",
            request.action
        ))),
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

    // debug!("{:?}", &json);

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

pub async fn load_tables() -> Result<String, Error> {
    let pool = Pool::new(URL)?;
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

pub async fn load_table(request: ApiRequest) -> Result<String, Error> {
    let data_string = &request.data.expect("No data sent for (load_table)");
    let (db, table): (String, String) =
        serde_json::from_str(data_string).expect("Invalid json object in request (load_table)");
    let pool = Pool::new(URL)?;
    let mut conn = pool.get_conn()?;
    let query = format!("SELECT * FROM {}.{} Limit 24", &db, &table);
    let results = conn.query(query)?;

    let response = TableData {
        db_name: db,
        table_name: table,
        count: results.len(),
        table_fields: table_fields_from(results),
    };

    Ok(send_json::<TableData>(ApiAction::LoadTable, response))
}

pub async fn run_query(request: ApiRequest) -> Result<String, Error> {
    let data_string = &request.data.expect("No data sent for (load_table)");
    let (db, table, query): (String, String, String) =
        serde_json::from_str(data_string).expect("Invalid json object in request (load_table)");
    let pool = Pool::new(URL)?;
    let mut conn = pool.get_conn()?;
    let query = format!("{}", query);
    let results = conn.query(query)?;

    let response = TableData {
        db_name: db,
        table_name: table,
        count: results.len(),
        table_fields: table_fields_from(results),
    };

    Ok(send_json::<TableData>(ApiAction::LoadTable, response))
}
