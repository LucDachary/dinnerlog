use log::error;
use mysql::prelude::*;
use mysql::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DBPOOL;

// Move into models.rs?
pub struct Happening {
    pub id: Uuid,
    pub when: PrimitiveDateTime,
    pub name: String,
    pub comment: Option<String>,
    pub created_on: PrimitiveDateTime,
    pub last_modified_on: PrimitiveDateTime,
}

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

pub fn fetch_happenings(last: u8) -> Vec<Happening> {
    let mut db_conn = DBPOOL
        .get_conn()
        .expect("Cannot obtain a connection to the database.");

    db_conn
        .query_map(
            format!(
                "SELECT id, date, name, comment, created_on, last_modified_on
            FROM happening
            ORDER BY created_on DESC
            LIMIT 0, {last}"
            ),
            |(id, when, name, comment, created_on, lmo)| Happening {
                id: Uuid::parse_str(String::from_utf8(id).expect("Cannot decode UTF8.").as_str())
                    .expect("Cannot decode UUID."),
                when,
                name,
                comment,
                created_on: created_on,
                last_modified_on: lmo,
            },
        )
        .expect("Cannot read the last happenings.")
}
