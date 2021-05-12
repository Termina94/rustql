use std::{borrow::Borrow, usize};

use crate::app::App;

impl App {
    pub fn get_table(&self, db_id: usize, table_id: usize) -> String {
        self
            .store
            .try_borrow()
            .expect("Cannot Access Store")
            .databases
            .get(db_id)
            .expect("Selected database does not exist")
            .tables
            .get(table_id)
            .expect("Selected table does not exist")
            .to_string()
    }

    pub fn get_db(&self, db_id: usize) -> String {
        self
            .store
            .try_borrow()
            .expect("Cannot Access Store")
            .databases
            .get(db_id)
            .expect("Can't Find Db")
            .name
            .to_string()
    }
}