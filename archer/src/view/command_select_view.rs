use cursive::{
    event::Key,
    utils::markup::StyledString,
    view::Nameable,
    views::{NamedView, OnEventView, ScrollView, SelectView},
};

use crate::model::command::Command;

static SEL_VIEW_NAME: &str = "CMD_SEL_VIEW::SEL_VIEW";

pub struct CommandSelectView {
    pub commands: Vec<Command>,
    pub view: OnEventView<ScrollView<NamedView<SelectView<Command>>>>,
}

impl CommandSelectView {
    pub fn new() -> Self {
        let mut cmd_list = OnEventView::new(ScrollView::new(
            SelectView::<Command>::new().with_name(SEL_VIEW_NAME),
        ));
        let mut sel_view = cmd_list.get_inner_mut().get_inner_mut().get_mut();
        sel_view.set_autojump(true);

        for i in 0..5 {
            sel_view.add_item(
                format!("command {}", i),
                Command {
                    label: format!("command {}", i),
                    shell: String::from("_"),
                    command: String::from("_"),
                    args: vec![],
                },
            );
        }

        cmd_list = cmd_list.on_event(Key::Enter, |s| {
            if let Some(mut sel_view) = s.find_name::<SelectView<Command>>(SEL_VIEW_NAME) {
                let _selection = sel_view.selection();
                // run a pipeline based on the selection
                let item_id = sel_view.selected_id().unwrap();
                if let Some((label, _value)) = sel_view.get_item_mut(item_id) {
                    *label = StyledString::plain("New Label");
                }

                log::info!(
                    "CommandSelectView::Selection = {}",
                    _selection.unwrap().label
                )
            }
        });

        let commands = vec![];
        Self {
            commands,
            view: cmd_list,
        }
    }
}

impl Default for CommandSelectView {
    fn default() -> Self {
        Self::new()
    }
}
