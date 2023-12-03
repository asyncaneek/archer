#![allow(unused)]
#![allow(clippy::needless_return)]

static EXPLORER_DIALOG: &str = "explorer_dialog";
static DIR_VIEW_NAME: &str = "dir_view";
static DIR_NAV_TO_PARENT: &str = "..";

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    env, fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use cursive::{
    backend::Backend,
    menu,
    theme::PaletteColor,
    view::{Nameable, Resizable, Scrollable},
    views::{Button, Dialog, LinearLayout, Panel, ScrollView, SelectView, TextArea, TextView},
    Cursive, CursiveExt, View,
};

use crate::state;

#[derive(Default)]
pub struct Tui {
    cursive: Cursive,
    path: PathBuf,
    // state: state::State,
}

impl Tui {
    pub fn new(state: state::State) -> Tui {
        let mut tui = Tui {
            cursive: Cursive::default(),
            path: state.working_directory.clone(),
        };

        tui.cursive.set_user_data(state);

        tui.initialize_cursive();
        tui.make_menu_bar();

        tui.make_content_area();
        tui.do_update_dir_list();

        return tui;
    }

    pub fn run(mut self) {
        self.cursive.run()
    }

    fn initialize_cursive(&mut self) {
        // load a theme from a file at runtime for fast development.
        self.cursive
            .load_theme_file("archer/assets/themes/dark.toml")
            .expect("failed to load theme");

        self.cursive.update_theme(|theme| {
            theme.palette[PaletteColor::Background] = cursive::theme::Color::TerminalDefault;
            theme.palette[PaletteColor::View] = cursive::theme::Color::TerminalDefault;
        });

        self.cursive.add_global_callback('q', |s| s.quit());

        // Start the Cursive UI event loop
        self.cursive.set_autorefresh(true);
    }

    fn make_content_area(&mut self) {
        let mut dir_list = SelectView::<PathBuf>::new()
            .on_submit(|siv: &mut Cursive, item: &PathBuf| {
                let state = siv.user_data::<state::State>().unwrap();
                state.working_directory = item.clone();
                // siv.add_layer(Dialog::text(item.to_str().unwrap()).button("Close", |s| {
                //     s.pop_layer();
                // }));
                siv.call_on_name(DIR_VIEW_NAME, |select_view: &mut SelectView<PathBuf>| {
                    update_dir_list(item.as_path(), select_view);
                });
                siv.call_on_name(EXPLORER_DIALOG, |dialog: &mut Dialog| {
                    dialog.set_title(item.to_str().unwrap_or("UNKNOWN"));
                });
            })
            .with_name(DIR_VIEW_NAME);

        let mut layout = LinearLayout::vertical().child(dir_list.scrollable());

        self.cursive.add_layer(
            Dialog::new()
                .title(self.path.to_str().unwrap())
                .content(layout)
                .padding_top(1)
                .with_name(EXPLORER_DIALOG)
                .full_screen(),
        );
    }

    fn make_menu_bar(&mut self) {
        self.cursive
            .menubar()
            .add_leaf("Archer", |_| {})
            .insert_delimiter(1);

        self.cursive.set_autohide_menu(false);
    }

    fn do_update_dir_list(&mut self) {
        self.cursive
            .call_on_name(DIR_VIEW_NAME, |select_view: &mut SelectView<PathBuf>| {
                update_dir_list(self.path.as_path(), select_view);
            });
    }
}
fn update_dir_list(directory: &Path, select_view: &mut SelectView<PathBuf>) {
    select_view.clear();
    // Read the contents of the directory.
    let entries = fs::read_dir(directory).unwrap();
    select_view.add_item(DIR_NAV_TO_PARENT, directory.parent().unwrap().to_owned());
    // Iterate over the directory entries and add directories to the SelectView.
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // Add the folder name to the SelectView.
            if let Some(folder_name) = path.file_name() {
                if let Some(folder_name_str) = folder_name.to_str() {
                    select_view.add_item(folder_name_str.to_owned(), path.clone());
                }
            }
        }
    }
}
