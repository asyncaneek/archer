#![allow(unused)]
#![allow(clippy::needless_return)]

use std::{env, fs};

use cursive::{
    menu,
    view::{Resizable, Scrollable},
    views::{Button, LinearLayout, Panel, ScrollView, SelectView, TextArea, TextView},
    Cursive, CursiveExt, View,
};

use crate::state;

#[derive(Default)]
pub struct Tui {
    siv: Cursive,
}

impl Tui {
    pub fn new(state: state::State) -> Tui {
        let mut siv = Cursive::default();

        let working_dir_str = state
            .working_directory
            .into_os_string()
            .into_string()
            .unwrap();

        Self::initialize_cursive(&mut siv);
        Self::make_menu_bar(&mut siv, &working_dir_str);
        siv.add_layer(
            cursive::views::Dialog::new()
                .content(Self::make_list_view(&working_dir_str))
                .button("Quit", |s| s.quit())
                .min_size((80, 50)),
        );

        // Self::make_content_area(&mut siv);

        return Tui { siv };
    }

    pub fn run(mut self) {
        self.siv.run()
    }

    fn initialize_cursive(siv: &mut Cursive) {
        // load a theme from a file at runtime for fast development.
        siv.load_theme_file("archer/assets/themes/dark.toml")
            .expect("failed to load theme");

        siv.add_global_callback('q', |s| s.quit());

        // Start the Cursive UI event loop
        siv.set_autorefresh(true);
    }

    fn make_content_area(siv: &mut Cursive) {
        siv.add_layer(
            LinearLayout::vertical()
                .child(TextView::new("Archer").full_screen().scrollable())
                .child(
                    Panel::new(TextArea::new().content(""))
                        .title("Archer")
                        .fixed_height(5),
                ),
        );
    }

    fn make_menu_bar(siv: &mut Cursive, heading: &str) {
        siv.menubar()
            .add_leaf("Archer", |_| {})
            .insert_delimiter(1)
            .add_leaf(heading, |_| {});

        siv.set_autohide_menu(false);
    }

    fn make_list_view(current_dir: &str) -> ScrollView<SelectView<String>> {
        // Read the contents of the directory.
        let entries = fs::read_dir(current_dir).unwrap();

        // Create a SelectView.
        let mut select_view = SelectView::new();

        select_view.add_item("..".to_owned(), current_dir.to_owned());
        // Iterate over the directory entries and add directories to the SelectView.
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                // Add the folder name to the SelectView.
                if let Some(folder_name) = path.file_name() {
                    if let Some(folder_name_str) = folder_name.to_str() {
                        select_view
                            .add_item(folder_name_str.to_owned(), folder_name_str.to_owned());
                    }
                }
            }
        }

        // Make the SelectView scrollable.
        let scrollable_select = ScrollView::new(select_view);
        return scrollable_select;
    }
}
