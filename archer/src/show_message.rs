use cursive::{views::Dialog, Cursive};

pub fn show_message(siv: &mut Cursive, message: &str) {
    siv.add_layer(Dialog::text(message).button("Close", |s| {
        s.pop_layer();
    }));
}
