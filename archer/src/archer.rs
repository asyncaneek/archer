use std::path::{self, PathBuf};

use crate::{
    detail::{self, name},
    state::State,
    view::{
        directory_select_view::{self, DirectorySelectView},
        main_view,
    },
};
use cursive::{
    event::{Event, MouseButton, MouseEvent},
    theme::PaletteColor,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, LinearLayout, SelectView},
    Cursive, CursiveExt,
};

#[derive(Default)]
pub struct Archer {
    pub cursive: Cursive,
    pub main_view: main_view::MainView,
}

impl Archer {
    pub fn new() -> Archer {
        cursive::logger::init();
        log::debug!("Logger initialized.");

        let mut state = State::default();
        let mut cursive = Cursive::default();

        Self::initialize_cursive(&mut cursive);
        // inject state into the ui
        cursive.set_user_data(state);

        // main view contains all ui interactions
        let mut main_view = main_view::MainView::new(&mut cursive);

        Archer { cursive, main_view }
    }

    fn initialize_cursive(cursive: &mut Cursive) {
        // load a theme from a file at runtime for fast development.
        cursive
            .load_theme_file("archer/assets/themes/dark.toml")
            .expect("failed to load theme");

        cursive.update_theme(|theme| {
            theme.palette[PaletteColor::Background] = cursive::theme::Color::TerminalDefault;
            theme.palette[PaletteColor::View] = cursive::theme::Color::TerminalDefault;
        });

        cursive.add_global_callback('q', |s| s.quit());
        cursive.add_global_callback('`', cursive::Cursive::toggle_debug_console);

        // start the cursive ui event loop
        cursive.set_autorefresh(true);
    }

    pub fn run(mut self) {
        self.cursive.run()
    }
}
