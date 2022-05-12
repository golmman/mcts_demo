use self::state::State;

mod common;
mod display;
mod eval;
mod movegen;
mod search;
mod state;

fn main() {
    let mut state = State::new();

    state.make_move(15);
    state.make_move(16);
    state.make_move(18);

    println!("{}", state);
}
