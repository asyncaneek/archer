use cursive::{
    event::{Event, EventResult},
    view::Scrollable,
    views::{ScrollView, SelectView},
    View,
};

use crate::model::command::Command;

pub struct CommandSelectView {
    pub commands: Vec<Command>,
    pub view: ScrollView<SelectView<Command>>,
}

impl CommandSelectView {
    pub fn new() -> Self {
        let mut cmd_list = SelectView::<Command>::new();
        cmd_list.set_autojump(true);

        for i in 0..5 {
            cmd_list.add_item(
                format!("command {}", i),
                Command {
                    label: String::from("_"),
                    shell: String::from("_"),
                    command: String::from("_"),
                    args: vec![],
                },
            );
        }

        let commands = vec![];
        Self {
            commands,
            view: cmd_list.scrollable(),
        }
    }
}

impl Default for CommandSelectView {
    fn default() -> Self {
        Self::new()
    }
}

impl View for CommandSelectView {
    fn draw(&self, printer: &cursive::Printer) {
        self.view.draw(printer);
    }
    fn layout(&mut self, xy: cursive::Vec2) {
        self.view.layout(xy);
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        self.view.on_event(event)
    }

    fn focus_view(
        &mut self,
        sel: &cursive::view::Selector,
    ) -> Result<cursive::event::EventResult, cursive::view::ViewNotFound> {
        self.view.focus_view(sel)
    }

    fn take_focus(
        &mut self,
        source: cursive::direction::Direction,
    ) -> Result<cursive::event::EventResult, cursive::view::CannotFocus> {
        self.view.take_focus(source)
    }
}
