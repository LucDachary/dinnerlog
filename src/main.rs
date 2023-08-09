use cursive::event;
use cursive::event::{Event, Key};
use cursive::menu;
use cursive::traits::*;
use cursive::views::{
    Button, Dialog, DummyView, EditView, LayerPosition, LinearLayout, Panel, SelectView, TextArea,
    TextView,
};
use cursive::Cursive;
use cursive::XY;
use log::{info, trace, LevelFilter};
use mysql::Pool;
use std::time::SystemTime;
use time::macros::{format_description, time};
use time::{Date, OffsetDateTime, PrimitiveDateTime};

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

    let vhappenings: SelectView<String> = SelectView::new().on_submit(|s, item: &String| {
        edit_happening(s, item);
    });

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
    siv.add_global_callback('~', Cursive::toggle_debug_console);
    siv.add_global_callback(Key::Esc, |s| {
        s.pop_layer();
        if s.screen().len() == 0 {
            trace!("There are no more layers, exiting…");
            s.quit();
        }
    });
    siv.set_autohide_menu(false);

    cursive::logger::init();
    log::set_max_level(LevelFilter::Info);

    list_last_happenings(&mut siv);

    // Vim-like navigation
    siv.add_global_callback('j', |s| s.on_event(Event::Key(Key::Down)));
    siv.add_global_callback('k', |s| s.on_event(Event::Key(Key::Up)));
    siv.add_global_callback('h', |s| s.on_event(Event::Key(Key::Left)));
    siv.add_global_callback('l', |s| s.on_event(Event::Key(Key::Right)));
    siv.add_global_callback('G', |s| s.on_event(Event::Key(Key::End)));

    siv.run();
}

/// Fetch and list last happenings.
fn list_last_happenings(s: &mut Cursive) {
    let happening_ids = sql::fetch_happenings(10);

    s.call_on_name("happenings", |vhappenings: &mut SelectView| {
        info!("Got last happenings view.");
        vhappenings.clear();
        for hid in happening_ids {
            vhappenings.add_item(
                format!("{} \u{2014} \u{201F}{}\u{201D}", hid.when.date(), hid.name),
                hid.id.to_string(),
            );
        }
    });
}

/// Open a dialog to edit the given happening.
fn edit_happening(s: &mut Cursive, happening_id: &String) {
    match sql::fetch_happening(happening_id) {
        Some(happening) => {
            s.add_layer(
                Dialog::around(make_happening_form(Some(&happening)))
                    .title("Edit")
                    .button("Save", move |s| {
                        let name: String = s
                            .call_on_name("h_name", |view: &mut EditView| view.get_content())
                            .unwrap()
                            .to_string();
                        let date: String = s
                            .call_on_name("h_date", |view: &mut EditView| view.get_content())
                            .unwrap()
                            .to_string();
                        let comment: String = s
                            .call_on_name("h_comment", |view: &mut TextArea| {
                                String::from(view.get_content())
                            })
                            .expect("Cannot find the comment's input.");

                        let mut new_happening = happening.clone();
                        new_happening.name = name;
                        new_happening.when = PrimitiveDateTime::new(
                            Date::parse(date.as_str(), format_description!("[year]-[month]-[day]"))
                                .unwrap(),
                            time!(0:00),
                        );
                        new_happening.comment = Some(comment);

                        let result = sql::update_happening(&new_happening);
                        if result.is_err() {
                            s.add_layer(Dialog::info(format!(
                                "Failed to update the entry: {}.",
                                result.unwrap_err()
                            )));
                        } else {
                            // info!("Happening {happening_id} has been updated.");
                            list_last_happenings(s);
                            s.pop_layer();
                        }
                    })
                    .button("Cancel", |s| {
                        s.pop_layer();
                    }),
                // TODO add a handler to UPDATE the Happening.
            );
        }
        None => s.add_layer(Dialog::info(
            "Cannot find this item. I cannot believe this is… happening!",
        )),
    }
}

/// Open a dialog with a Happening form.
fn add_happening(s: &mut Cursive) {
    s.add_layer(
        Dialog::around(make_happening_form(None))
            .title("Add a happening")
            .button("Add", |s| {
                let name = s
                    .call_on_name("h_name", |view: &mut EditView| view.get_content())
                    .unwrap();
                let date = s
                    .call_on_name("h_date", |view: &mut EditView| view.get_content())
                    .unwrap();
                let comment: Option<String> = s.call_on_name("h_comment", |view: &mut TextArea| {
                    String::from(view.get_content())
                });
                // TODO add other fields.
                insert_happening(&name, &date, comment);

                // TODO inform about the success
                s.pop_layer();

                list_last_happenings(s);
            })
            .button("Cancel", |s| {
                s.pop_layer();
            }),
    );
}

/// Build and return a form to create or edit a happening.
fn make_happening_form(happening: Option<&sql::Happening>) -> LinearLayout {
    let now = SystemTime::now();
    // let now_str: OffsetDateTime = now.into().format(&Rfc3339);
    let now_str = <SystemTime as Into<OffsetDateTime>>::into(now)
        .format(format_description!("[year]-[month]-[day]"))
        .unwrap();

    LinearLayout::vertical()
        .child(TextView::new("Name"))
        .child(
            EditView::new()
                .content(match happening {
                    Some(h) => h.name.as_str(),
                    None => "",
                })
                .max_content_width(100)
                .with_name("h_name"),
        )
        .child(TextView::new("Date (yyyy-mm-dd)"))
        .child(
            EditView::new()
                .content(match happening {
                    Some(happening) => happening
                        .when
                        .format(format_description!("[year]-[month]-[day]"))
                        .unwrap(),
                    None => now_str,
                })
                .max_content_width(10)
                .with_name("h_date"),
        )
        .child(TextView::new("Comment"))
        .child(
            TextArea::new()
                .content(match happening {
                    Some(h) => h.comment.clone().unwrap_or(String::new()),
                    None => String::new(),
                })
                // Here we still have a TextArea
                .with_name("h_comment")
                // Here we have a NamedView
                .min_height(3)
                // Here we have a ResizedView
                .fixed_width(30),
        )
}
