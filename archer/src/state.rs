use std::path::{Path, PathBuf};

pub struct State {
    pub working_directory: PathBuf,
}

impl State {
    pub fn new(working_directory: Option<&Path>) -> Self {
        Self {
            working_directory: working_directory
                .map_or_else(Self::get_current_working_dir, PathBuf::from),
        }
    }

    fn get_current_working_dir() -> PathBuf {
        match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => match dirs::home_dir() {
                Some(home_dir) => home_dir,
                None => PathBuf::from("FAILED"),
            },
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            working_directory: State::get_current_working_dir(),
        }
    }
}
