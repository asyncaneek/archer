use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{DummyView, LinearLayout, OnEventView, Panel},
    Cursive,
};

use crate::{name, state};

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

        let update_title = |s: &mut Cursive| {
            if let Some(mut dir_sel_view) =
                s.find_name::<Panel<DirectorySelectView>>(name::DIR_SEL_VIEW)
            {
                log::info!("update_title");
                let path = dir_sel_view.get_inner_mut().path.clone();
                dir_sel_view.set_title(path.file_name().unwrap().to_str().unwrap());
            }
        };
        // directory_select_view
        let dir_sel_view = OnEventView::new(
            Panel::new(DirectorySelectView::new(path)).with_name(name::DIR_SEL_VIEW),
        )
        .on_event(Key::Enter, update_title);

        let cmd_sel_view = Panel::new(DummyView).title("Commands");
        let queued_cmd_view = Panel::new(DummyView).title("Queue");
        let layout = LinearLayout::horizontal()
            .child(
                LinearLayout::vertical()
                    .child(dir_sel_view.full_screen())
                    .child(cmd_sel_view.full_screen()),
            )
            .child(DummyView {}.fixed_width(1))
            .child(queued_cmd_view.full_screen());
        cursive.add_layer(layout);
        update_title(cursive);

        // command_select_view
        // command_output_view
        // status_bar_view

        return MainView {};
    }
}
