use cursive::{
    view::{Nameable, Resizable},
    views::{Dialog, LinearLayout, Panel},
    Cursive,
};

use crate::detail::name;
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
        let dir_sel_view = Panel::new(DirectorySelectView::new(path)).full_screen();
        let cmd_sel_view = Panel::new(DirectorySelectView::new(path)).full_screen();
        let layout = LinearLayout::horizontal()
            .child(dir_sel_view)
            .child(cmd_sel_view);

        cursive.add_layer(
            Dialog::new()
                .title(path.to_str().unwrap())
                .content(layout)
                .with_name(name::MAIN_VIEW)
                .full_screen(),
        );

        // command_select_view
        // command_output_view
        // status_bar_view

        return MainView {};
    }
}
