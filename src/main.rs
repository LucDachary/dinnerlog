use cursive::align::HAlign;
use cursive::event;
use cursive::menu;
use cursive::traits::*;
use cursive::view::Offset;
use cursive::views::{
    Button, DebugView, Dialog, DummyView, EditView, LayerPosition, LinearLayout, ListView, Panel,
    TextArea, TextView,
};
use cursive::Cursive;
use cursive::XY;
use log::{debug, error, LevelFilter};
use mysql::prelude::*;
use mysql::Pool;
use mysql::Value;
use mysql::*;
use std::time::SystemTime;
use time::format_description::well_known::Rfc3339;
use time::macros::date;
use time::macros::format_description;
use time::Date;
use time::OffsetDateTime;
use time::PrimitiveDateTime;
use uuid::Uuid;

use sql::insert_happening;
pub mod sql;

#[macro_use]
extern crate lazy_static;
struct Happening {
    id: Uuid,
    when: PrimitiveDateTime,
    name: String,
    comment: Option<String>,
    created_on: PrimitiveDateTime,
    last_modified_on: PrimitiveDateTime,
}

// TODO get credentials from environment variables
// This command assumes the database is listening on the same host, or with a Docker port
// share.
const DB_URL: &str = "mysql://dbuser:dbpassword@localhost:3306/dinnerlog";
lazy_static! {
    static ref DBPOOL: Pool =
        Pool::new(DB_URL).expect("Cannot obtain a connection pool to the database.");
}

fn main() {
    // Fetch last happenings
    let mut db_conn = DBPOOL
        .get_conn()
        .expect("Cannot obtain a connection to the database.");

    // DEV
    let happening_ids = db_conn
        .query_map(
            "SELECT id, date, name, comment, created_on, last_modified_on
            FROM happening
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

    siv.load_theme_file("assets/theme.toml").unwrap();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('n', add_happening);

    let mut vhappenings = ListView::new();
    for hid in happening_ids {
        vhappenings.add_child(
            format!("{} \u{2014}", hid.when.date()).as_str(),
            TextView::new(format!("\u{201F}{}\u{201D}", hid.name).as_str()),
        );
    }

    let page = LinearLayout::vertical()
        .child(TextView::new("Dinner Log").center())
        .child(DummyView.fixed_height(1))
        .child(Panel::new(vhappenings).title("Last happenings"))
        .child(TextView::new(
            "Press 'q' to exit. Press 'F1' to select the menu bar.",
        ))
        .child(Button::new("Quit", |s| s.quit()))
        .full_screen();

    let layout = LinearLayout::horizontal()
        .child(page)
        // DEV
        .child(Panel::new(DebugView::new()).title("Log"));

    siv.add_layer(layout);
    // TODO center the layer
    siv.reposition_layer(LayerPosition::FromBack(0), XY::center());

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

    siv.add_global_callback('~', Cursive::toggle_debug_console);
    cursive::logger::init();
    log::set_max_level(LevelFilter::Warn);

    siv.run();
}

/// Open a dialog with a Happening form.
fn add_happening(s: &mut Cursive) {
    let now = SystemTime::now();
    // let now_str: OffsetDateTime = now.into().format(&Rfc3339);
    let now_str = <SystemTime as Into<OffsetDateTime>>::into(now)
        .format(format_description!("[year]-[month]-[day]"))
        .unwrap();

    s.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Name"))
                .child(EditView::new().max_content_width(100).with_name("h_name"))
                .child(TextView::new("Date (yyyy-mm-dd)"))
                .child(
                    EditView::new()
                        .content(now_str)
                        .max_content_width(10)
                        .with_name("h_date"),
                )
                .child(TextView::new("Comment"))
                .child(TextArea::new().min_height(3).fixed_width(30)),
        )
        .title("Add a happening")
        .button("Add", |s| {
            let name = s
                .call_on_name("h_name", |view: &mut EditView| view.get_content())
                .unwrap();
            let date = s
                .call_on_name("h_date", |view: &mut EditView| view.get_content())
                .unwrap();
            // TODO add other fields.
            insert_happening(&name, &date);

            // TODO inform about the success
            s.pop_layer();
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}
