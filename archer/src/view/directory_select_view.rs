use core::slice;
use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
};

use cursive::{
    view::{Nameable, Scrollable},
    views::{Dialog, NamedView, ScrollView, SelectView, ViewRef},
    Cursive, View,
};

use cursive::event::{Event, EventResult, Key};

use crate::{
    detail::{self, name},
    state,
};

pub struct DirectorySelectView {
    name: &'static str,
    path: PathBuf,
    pub view: SelectView<PathBuf>,
}

impl View for DirectorySelectView {
    fn draw(&self, printer: &cursive::Printer) {
        self.view.draw(printer);
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Enter) => {
                if let Some(selection) = self.view.selection() {
                    self.path = selection.to_owned().to_path_buf();
                    self.update_entries(selection.as_path());
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

        let mut dir_sel_view = DirectorySelectView {
            name: name::DIR_SEL_VIEW,
            path: working_directory.to_owned(),
            view: dir_list,
        };
        dir_sel_view.update_entries(working_directory);

        return dir_sel_view;
    }
    fn update_entries(&mut self, directory: &Path) {
        log::trace!("[DirectorySelectView::update_entries]");
        let select_view = &mut self.view;

        select_view.clear();
        select_view.add_item(
            name::DIR_NAV_TO_PARENT,
            directory.parent().unwrap_or(directory).to_owned(),
        );

        if let Ok(entries) = fs::read_dir(directory) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(folder_name) = path.file_name() {
                        if let Some(folder_name_str) = folder_name.to_str() {
                            select_view.add_item(folder_name_str.to_owned(), path.clone());
                        }
                    }
                }
            }
        }
    }
}
