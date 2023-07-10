use cursive::views::{Button, LinearLayout, Panel, TextView};

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    let page = LinearLayout::vertical()
        .child(TextView::new("Dinner Log").center())
        .child(Panel::new(TextView::new("TODO")).title("Last happenings"))
        .child(TextView::new("Press 'q' to exit."))
        .child(Button::new("Quit", |s| s.quit()));

    siv.add_layer(page);
    siv.run();
}
