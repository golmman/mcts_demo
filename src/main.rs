use crate::movegen::Move;

use self::state::State;

mod common;
mod display;
mod eval;
mod movegen;
mod search;
mod state;

fn main() {
    let mut state = State::new();

    state.make_move(Move(15));
    state.make_move(Move(16));
    state.make_move(Move(18));

    println!("{}", state);
}
