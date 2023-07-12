use cursive::align::HAlign;
use cursive::event;
use cursive::menu;
use cursive::views::{Button, DebugView, Dialog, LinearLayout, ListView, Panel, TextView};
use mysql::prelude::*;
use mysql::Pool;
use mysql::Value;
use time::macros::date;
use time::Date;
use time::PrimitiveDateTime;
use uuid::Uuid;

struct Happening {
    id: Uuid,
    when: PrimitiveDateTime,
    name: String,
    comment: Option<String>,
    created_on: PrimitiveDateTime,
    last_modified_on: PrimitiveDateTime,
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
        .query_map(
            "SELECT id, date, name, comment, created_on, last_modified_on
            FROM happening_happening
            ORDER BY created_on DESC
            LIMIT 0, 10",
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
        .expect("Cannot read the last happenings.");

    // Draw the TUI
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let mut vhappenings = ListView::new();
    for hid in happening_ids {
        vhappenings.add_child(
            format!("{} \u{2014}", hid.when.date()).as_str(),
            TextView::new(format!("\u{201F}{}\u{201D}", hid.name).as_str()),
        );
    }

    let page = LinearLayout::vertical()
        .child(TextView::new("Dinner Log").center())
        .child(Panel::new(vhappenings).title("Last happenings"))
        .child(TextView::new(
            "Press 'q' to exit. Press 'F1' to select the menu bar.",
        ))
        .child(Button::new("Quit", |s| s.quit()))
        // DEV
        .child(DebugView::new());

    siv.add_layer(page);

    // Menu
    siv.menubar()
        .add_subtree(
            "Happenings",
            menu::Tree::new().leaf("New", |s| s.add_layer(Dialog::info("New happening!"))),
        )
        .add_subtree(
            "Help",
            menu::Tree::new().leaf("Shortcuts", |s| {
                s.add_layer(Dialog::info(
                    r"Press 'q' to exit. Press 'F1' to open the menu.",
                ))
            }),
        );

    siv.add_global_callback(event::Key::F1, |s| s.select_menubar());
    siv.set_autohide_menu(false);

    siv.run();
}
