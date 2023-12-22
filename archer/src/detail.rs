use std::{
    fs,
    path::{Path, PathBuf},
};

use cursive::{
    views::{Dialog, SelectView},
    Cursive,
};

pub mod name {
    pub static MAIN_VIEW: &str = "explorer_dialog";

    pub static DIR_SEL_VIEW: &str = "dir_view";

    pub static DIR_NAV_TO_PARENT: &str = "..";
}

pub fn show_message(siv: &mut Cursive, message: &str) {
    siv.add_layer(Dialog::text(message).button("Close", |s| {
        s.pop_layer();
    }));
}
