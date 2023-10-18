use cursive::views::{LinearLayout, ResizedView, TextView, ListView};

fn main() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("./theme.toml")).unwrap();

    siv.add_global_callback('q', |s| s.quit());

    // Create the left view with a list view
    let list_view = ListView::new()
    .child("cA", TextView::new("command A"))
    .child("cB", TextView::new("command B"));
    let left_view = ResizedView::with_full_height(list_view);

    // Create the right view with a text view
    let text_view = TextView::new("Text View");
    let right_view = ResizedView::with_full_screen(text_view);

    // Create the vertically split screen view
    let split_view = LinearLayout::horizontal().child(left_view).child(right_view);

    // Add the full-screen view to the Cursive root
    siv.add_fullscreen_layer(split_view);

    siv.run();
}
