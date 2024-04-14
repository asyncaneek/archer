use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{DummyView, LinearLayout, OnEventView, Panel},
    Cursive,
};

use crate::{model::name, state};

use super::{directory_select_view::DirectorySelectView, pipeline_select_view::PipelineSelectView};
use crate::view::command_select_view::CommandSelectView;

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

        let on_cwd_changed = |cursive: &mut Cursive| {
            if let Some(mut dir_sel_view) =
                cursive.find_name::<Panel<DirectorySelectView>>(name::DIR_SEL_VIEW)
            {
                log::info!("update_title");
                let path = dir_sel_view.get_inner_mut().path.clone();
                let path_str = path.to_string_lossy().into_owned();

                // mutate state
                let state = cursive.user_data::<state::State>().unwrap();
                state.working_directory = path;
                log::info!(
                    "state.working_directory = {}",
                    state.working_directory.to_string_lossy().into_owned()
                );

                // update dialog title
                dir_sel_view.set_title(path_str);
            }
        };

        // directory_select_view
        let dir_sel_view = OnEventView::new(
            Panel::new(DirectorySelectView::new(path)).with_name(name::DIR_SEL_VIEW),
        )
        .on_event(Key::Enter, on_cwd_changed);

        // command_select_view
        let cmd_sel_view = Panel::new(CommandSelectView::new().view)
            .title("commands")
            .with_name(name::CMD_SEL_VIEW);

        // pipeline_view
        let queued_cmd_view = Panel::new(PipelineSelectView::new().view)
            .title("pipeline")
            .with_name(name::CMD_SEL_VIEW);

        // layout
        let layout = LinearLayout::horizontal()
            .child(
                LinearLayout::vertical()
                    .child(dir_sel_view.full_screen())
                    .child(cmd_sel_view.full_screen()),
            )
            .child(DummyView {}.fixed_width(1))
            .child(queued_cmd_view.full_screen());

        cursive.add_layer(layout);

        // refresh title
        on_cwd_changed(cursive);

        return MainView {};
    }
}
