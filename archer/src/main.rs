use cursive::view::Nameable;
use cursive::views::{TextContent, TextView};
use cursive::{Cursive, CursiveExt, View};
use std::thread;

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(make_content_area() );

    // Start the Cursive UI event loop
    siv.set_autorefresh(true);
    siv.run();
}

fn make_content_area() -> Box<dyn View> {
    // Create a shared mutable state to hold the text
    let content = TextContent::new("");

    // Clone the shared state for use in the thread
    let thread_shared_text = content.clone();

    // Spawn a new thread to update the text asynchronously
    thread::spawn(move || {
        // Simulate some asynchronous process
        thread::sleep(std::time::Duration::from_secs(3));

        // Update the text in the shared state
        thread_shared_text.append("\nYeet! It worked 🖖");
    });

    // Create a TextView and add it to the Cursive UI
    let text_view = TextView::new_with_content(content.clone()).with_name("content_area");
    Box::new(text_view)
}
