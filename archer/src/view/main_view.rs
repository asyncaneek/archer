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

use crate::detail::{self, name};
use crate::state;

use super::directory_select_view::DirectorySelectView;

#[derive(Default)]
pub struct MainView;

impl MainView {
    pub fn new(cursive: &mut Cursive) -> MainView {
        let state = cursive.user_data::<state::State>();
        let path = &state.unwrap().working_directory.clone();

        // menu_bar_view
        cursive
            .menubar()
            .add_leaf("Archer", |_| {})
            .add_leaf("Debug", cursive::Cursive::toggle_debug_console)
            .insert_delimiter(1);

        cursive.set_autohide_menu(false);

        // directory_select_view
        let mut layout = DirectorySelectView::new(path);
        cursive.add_layer(
            Dialog::new()
                .title(path.to_str().unwrap())
                .content(layout)
                .padding_top(2)
                .with_name(name::MAIN_VIEW)
                .full_screen(),
        );

        // command_select_view
        // command_output_view
        // status_bar_view

        return MainView {};
    }
}
