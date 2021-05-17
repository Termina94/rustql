use mysql::{from_value, Row};
use rustql_types::{TableField, TableFields};
use std::{any::Any, collections::HashMap};

// This cannot live in the api_types lib as it cannot compile to wasm
pub fn table_fields_from(results: Vec<Row>) -> TableFields {
    let mut fields: Vec<TableField> = vec![];

    results.iter().for_each(|row| {
        row.columns_ref()
            .iter()
            .enumerate()
            .for_each(|(index, col)| {
                let field_name = col.name_str().as_ref().to_string();
                let field_value = row[col.name_str().as_ref()].clone();

                if let Some(field_values) = fields.get_mut(index) {
                    let value: String = match &field_value {
                        mysql::Value::NULL => String::from("null"),
                        _ => from_value(field_value),
                    };
                    field_values.values.push(value);
                } else {
                    let value: String = match &field_value {
                        mysql::Value::NULL => String::from("null"),
                        _ => from_value(field_value),
                    };

                    fields.insert(
                        index,
                        TableField {
                            name: field_name,
                            sql_type: String::from("String"),
                            values: vec![value],
                        },
                    );
                }
            });
    });

    fields
}
