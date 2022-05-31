use std::ops::Range;

use crate::common::rand::Random;
use crate::movegen::Move;

use self::search::Search;
use self::state::State;

mod common;
mod display;
mod eval;
mod movegen;
mod search;
mod state;

fn main() {
    let mut state = State::new();

    state.search();

    println!("{:?}", state.tree);
    println!("{}", state);

    let mut rand = Random::new(0);

    //state.make_move(Move(15));
    //state.make_move(Move(16));
    //state.make_move(Move(18));
    //println!("{}", state);

    //state.unmake_move();
    //state.unmake_move();
    //println!("{}", state);
}
