// #![allow(unused)]
#![allow(clippy::needless_return)]

pub mod archer;
pub mod state;
pub mod view;
pub mod model;

use archer::Archer;

fn main() {
    let archer = Archer::new();
    archer.run();
}
