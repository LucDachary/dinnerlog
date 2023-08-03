use log::error;
use mysql::prelude::*;
use mysql::*;
use uuid::Uuid;

use crate::DBPOOL;

pub fn insert_happening(name: &str, date: &str) {
    let mut db_conn = DBPOOL
        .get_conn()
        .expect("Cannot obtain a connection to the database.");

    match db_conn.exec_drop(
        r"INSERT INTO happening (id, name, date, created_on, last_modified_on)
          VALUES (:id, :name, DATE(:date), NOW(), NOW())",
        params! {
            "id" => Uuid::new_v4().as_simple().to_string(),
            name,
            "date" => date,
        },
    ) {
        // TODO handle errors in this function rather than log and exit.
        Err(e) => error!("{}", e),
        Ok(_) => (),
    };
}
