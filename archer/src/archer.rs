use crate::{state::State, tui::Tui};

#[derive(Default)]
pub struct Archer {
    pub tui: Tui,
}

impl Archer {
    pub fn new() -> Archer {
        Archer {
            tui: Tui::new(State::default()),
        }
    }
}
