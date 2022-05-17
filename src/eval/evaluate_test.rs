use std::ops::Add;

use crate::common::{BOARD_SIZE, BOARD_WIDTH};
use crate::eval::Eval;
use crate::movegen::Move;
use crate::state::{Board, PieceType, State};

fn create_board(b: [u8; BOARD_SIZE]) -> Board {
    let mut board = [None; BOARD_SIZE];

    for i in 0..BOARD_SIZE {
        board[i] = match b[i] {
            0 => None,
            1 => Some(PieceType::Black),
            2 => Some(PieceType::White),
            _ => panic!("piece number should be one of 0, 1, 2"),
        };
    }

    board
}

//impl Into<Move> for (u8, u8) {
//    fn into(self) -> Move {
//        BOARD_WIDTH * self.1 as usize + self.0 as usize
//    }
//}

#[rustfmt::skip]
const TEST_BOARD: [u8; BOARD_SIZE] = [
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
];

#[test]
fn it_draws_overlines() {
    let mut state = State::new();

    state.board = create_board(TEST_BOARD);

    assert_eq!(state.evaluate(Move(50)), 1.0);
    assert_eq!(state.evaluate(Move(51)), 1.0);
    assert_eq!(state.evaluate(Move(52)), 1.0);
    assert_eq!(state.evaluate(Move(53)), 1.0);
    assert_eq!(state.evaluate(Move(54)), 1.0);
}

fn it_recognizes_horizontal_black_wins() {
    let mut state = State::new();

    state.board = create_board(TEST_BOARD);

    assert_eq!(state.evaluate(Move(50)), 1.0);
    assert_eq!(state.evaluate(Move(51)), 1.0);
    assert_eq!(state.evaluate(Move(52)), 1.0);
    assert_eq!(state.evaluate(Move(53)), 1.0);
    assert_eq!(state.evaluate(Move(54)), 1.0);
}
