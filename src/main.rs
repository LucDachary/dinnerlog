use cursive::event;
use cursive::menu;
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LayerPosition, LinearLayout, ListView, Panel, TextArea,
    TextView,
};
use cursive::Cursive;
use cursive::XY;
use log::{info, LevelFilter};
use mysql::Pool;
use std::time::SystemTime;
use time::macros::format_description;
use time::OffsetDateTime;

use sql::insert_happening;
pub mod sql;

#[macro_use]
extern crate lazy_static;

// TODO get credentials from environment variables
// This command assumes the database is listening on the same host, or with a Docker port
// share.
const DB_URL: &str = "mysql://dbuser:dbpassword@localhost:3306/dinnerlog";
lazy_static! {
    static ref DBPOOL: Pool =
        Pool::new(DB_URL).expect("Cannot obtain a connection pool to the database.");
}

fn main() {
    // Draw the TUI
    let mut siv = cursive::default();

    siv.load_theme_file("assets/theme.toml").unwrap();

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('n', add_happening);

    let vhappenings = ListView::new();

    let page = LinearLayout::vertical()
        .child(TextView::new("Dinner Log").center())
        .child(DummyView.fixed_height(1))
        .child(Panel::new(vhappenings.with_name("happenings")).title("Last happenings"))
        .child(TextView::new(
            "Press 'q' to exit. Press 'F1' to select the menu bar.",
        ))
        .child(Button::new("Quit", |s| s.quit()));

    let layout = LinearLayout::horizontal().child(page);

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
    log::set_max_level(LevelFilter::Info);

    list_last_happenings(&mut siv);

    siv.run();
}

/// Fetch and list last happenings.
fn list_last_happenings(s: &mut Cursive) {
    let happening_ids = sql::fetch_happenings(10);

    s.call_on_name("happenings", |vhappenings: &mut ListView| {
        info!("Got last happenings view.");
        vhappenings.clear();
        for hid in happening_ids {
            vhappenings.add_child(
                format!("{} \u{2014}", hid.when.date()).as_str(),
                TextView::new(format!("\u{201F}{}\u{201D}", hid.name).as_str()),
            );
        }
    });
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

            list_last_happenings(s);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}
