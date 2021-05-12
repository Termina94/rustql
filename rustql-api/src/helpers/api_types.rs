use std::collections::HashMap;
use mysql::{Row, from_value};
use rustql_types::{TableFieldData, TableFields};



// This cannot live in the api_types lib as it cannot compile to wasm
pub fn table_fields_from(results: Vec<Row>) -> TableFields {

    let mut fields: HashMap<String, TableFieldData> = HashMap::new();

    results.iter().for_each(|row| {

        row.columns_ref().iter().for_each(|col| {

            let field_name = col.name_str().as_ref().to_string();
            let field_value = row[col.name_str().as_ref()].clone();

            if let Some(field_values) = fields.get_mut(&field_name) {
                field_values.values.push(from_value(field_value));
            } else {
                fields.insert(field_name, TableFieldData {
                    field_type: String::from("String"),
                    values: vec![from_value(field_value)]
                });
            }
        });
    });
    
    fields
}