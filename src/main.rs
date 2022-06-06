use std::ops::Range;

use crate::common::rand::Random;
use crate::movegen::Move;

use self::eval::Eval;
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
    state.make_move(Move(2));
    state.make_move(Move(14));
    state.make_move(Move(3));
    state.make_move(Move(130));
    state.make_move(Move(4));
    state.make_move(Move(12));
    //state.make_move(Move(5));
    //state.make_move(Move(11));

    let ev = state.evaluate();
    println!("{:?}", ev);

    state.search();

    //println!("{:?}", state.tree);
    //println!("{}", state);

    let mut rand = Random::new(0);

    //println!("{}", state);

    //state.unmake_move();
    //state.unmake_move();
    //println!("{}", state);
}
