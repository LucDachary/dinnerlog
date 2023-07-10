use cursive::views::{Button, LinearLayout, Panel, TextView};
use mysql::prelude::*;
use mysql::Pool;
use mysql::Value::Bytes;
use uuid::Uuid;

struct Happening {
    id: Uuid,
    // TODO use datetime type?
    date: String,
    name: String,
    comment: Option<String>,
    // TODO use datetime type?
    created_on: String,
    // TODO use datetime type?
    last_modified_on: String,
}

fn main() {
    // Fetch last happenings
    // TODO get credentials from environment variables
    // This command assumes the database is listening on the same host, or with a Docker port
    // share.
    let db_url = "mysql://dbuser:dbpassword@localhost:3306/dinnerlog";
    let db_pool = Pool::new(db_url).expect("Cannot obtain a connection pool to the database.");
    let mut db_conn = db_pool
        .get_conn()
        .expect("Cannot obtain a connection to the database.");
    // DEV
    let happening_ids = db_conn
        .query_map::<(Vec<u8>, Vec<u8>), _, &str, Happening>(
            "SELECT id, name FROM happening_happening ORDER BY created_on DESC LIMIT 0, 10",
            |(id, name)| Happening {
                id: Uuid::parse_str(String::from_utf8(id).expect("Cannot decode UTF8.").as_str())
                    .expect("Cannot decode UUID."),
                date: String::new(),
                name: String::from_utf8(name).expect("Cannot decode UTF8."),
                comment: Some(String::new()),
                created_on: String::new(),
                last_modified_on: String::new(),
            },
        )
        .expect("Cannot read the last happenings.");

    // Draw the TUI
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let mut vhappenings = LinearLayout::vertical();
    for hid in happening_ids {
        vhappenings = vhappenings.child(TextView::new(format!("{}", hid.name)));
    }

    let page = LinearLayout::vertical()
        .child(TextView::new("Dinner Log").center())
        .child(Panel::new(vhappenings).title("Last happenings"))
        .child(TextView::new("Press 'q' to exit."))
        .child(Button::new("Quit", |s| s.quit()));

    siv.add_layer(page);
    siv.run();
}
