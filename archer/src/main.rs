#![allow(unused)]
#![allow(clippy::needless_return)]

pub mod archer;
pub mod detail;
pub mod state;
pub mod view;

use archer::Archer;

fn main() {
    let archer = Archer::new();
    archer.run();
}
