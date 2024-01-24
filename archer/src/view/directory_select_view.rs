use std::{
    fs,
    path::{Path, PathBuf},
};

use cursive::{
    view::Scrollable,
    views::{ScrollView, SelectView},
    View,
};

use cursive::event::{Event, EventResult, Key};

use crate::detail::name;

pub struct DirectorySelectView {
    pub name: &'static str,
    pub path: PathBuf,
    pub view: ScrollView<SelectView<PathBuf>>,
}

impl View for DirectorySelectView {
    fn draw(&self, printer: &cursive::Printer) {
        self.view.draw(printer);
    }
    fn layout(&mut self, xy: cursive::Vec2) {
        self.view.layout(xy);
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Enter) => {
                if let Some(selection) = self.view.get_inner().selection() {
                    self.path = selection.to_owned().to_path_buf();
                    self.update_entries(selection.as_path());
                    self.layout(cursive::XY { x: 0, y: 0 });
                }
                EventResult::Consumed(None)
            }
            _ => self.view.on_event(event),
        }
    }
}

impl DirectorySelectView {
    pub fn new(working_directory: &Path) -> Self {
        let mut dir_list = SelectView::<PathBuf>::new();
        dir_list.set_autojump(true);

        let mut dir_sel_view = DirectorySelectView {
            name: name::DIR_SEL_VIEW,
            path: working_directory.to_owned(),
            view: dir_list.scrollable(),
        };
        dir_sel_view.update_entries(working_directory);

        return dir_sel_view;
    }
    fn update_entries(&mut self, directory: &Path) {
        log::trace!("[DirectorySelectView::update_entries]");
        let select_view = &mut self.view;

        select_view.get_inner_mut().clear();
        select_view.get_inner_mut().add_item(
            name::DIR_NAV_TO_PARENT,
            directory.parent().unwrap_or(directory).to_owned(),
        );

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(folder_name) = path.file_name() {
                        if let Some(folder_name_str) = folder_name.to_str() {
                            select_view
                                .get_inner_mut()
                                .add_item(folder_name_str.to_owned(), path.clone());
                        }
                    }
                }
            }
        }
    }
}
