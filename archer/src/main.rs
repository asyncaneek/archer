#![allow(unused)]

pub mod archer;
pub mod state;
pub mod tui;

use archer::Archer;

fn main() {
    let archer = Archer::new();

    archer.tui.run();
}
