use log::error;
use mysql::prelude::*;
use mysql::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DBPOOL;

// Move into models.rs?
#[derive(Clone)]
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
                    .expect("Cannot decode this UUID."),
                when,
                name,
                comment,
                created_on: created_on,
                last_modified_on: lmo,
            },
        )
        .expect("Cannot read the last happenings.")
}

pub fn fetch_happening(id: &String) -> Option<Happening> {
    let mut db_conn = DBPOOL
        .get_conn()
        .expect("Cannot obtain a connection to the database.");

    let rows = db_conn
        .exec_map(
            "SELECT id, date, name, comment, created_on, last_modified_on
            FROM happening
            WHERE id = ?",
            (id.replace("-", ""),),
            |row: Row| Happening {
                id: Uuid::parse_str(
                    String::from_utf8(row.get(0).unwrap())
                        .expect("Cannot decode UTF8.")
                        .as_str(),
                )
                .expect("Cannot decode this UUID."),
                when: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                comment: row.get(3).unwrap(),
                created_on: row.get(4).unwrap(),
                last_modified_on: row.get(5).unwrap(),
            },
        )
        .expect("Cannot fetch this happening.");

    match rows.len() {
        0 => None,
        1 => Some(rows[0].clone()),
        _ => panic!("Found more than 1 happening."),
    }
}
