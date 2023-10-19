use cursive::view::Nameable;
use cursive::views::{TextContent, TextView};
use cursive::{Cursive, CursiveExt, View};

use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::thread;

fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(make_content_area());

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
        let _ = run_command(thread_shared_text.clone());
        // thread_shared_text.append("\nYeet! It worked 🖖");
    });

    // Create a TextView and add it to the Cursive UI
    let text_view = TextView::new_with_content(content.clone()).with_name("content_area");
    Box::new(text_view)
}

fn run_command(text_content: TextContent) -> Result<(), Box<dyn std::error::Error>> {
    let stdout = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    let reader = BufReader::new(stdout);
    reader.lines().for_each(move |line| {
        text_content.append("\n".to_string());
        text_content.append(line.unwrap_or("-".to_string()));
    });
    Ok(())
}
