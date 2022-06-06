use crate::common::point2d::Point2D;
use crate::common::BOARD_SIZE;
use crate::eval::{Evaluation::Draw, Evaluation::WinPlayer1, Evaluation::WinPlayer2};
use crate::movegen::Move;
use crate::state::{Board, PieceType, State};

use super::{Evaluation, Eval};

#[rustfmt::skip]
const TEST_BOARD: [u8; BOARD_SIZE] = [
    2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    1, 0, 2, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    1, 0, 0, 2, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
    1, 0, 0, 0, 2, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn create_board(b: [u8; BOARD_SIZE]) -> Board {
    let mut board = [None; BOARD_SIZE];

    for i in 0..BOARD_SIZE {
        board[i] = match b[i] {
            0 => None,
            1 => Some(PieceType::Player1),
            2 => Some(PieceType::Player2),
            _ => panic!("piece number should be one of 0, 1, 2"),
        };
    }

    board
}

fn evaluate_at(piece_type: PieceType, x: i8, y: i8) -> Evaluation {
    let mut state = State::new();

    state.board = create_board(TEST_BOARD);

    state.moves.append(&mut vec![Move(0); 8]);
    if piece_type == PieceType::Player2 {
        state.moves.push(Move(0));
    }
    state.moves.push(Move::from(Point2D(x, y)));

    state.evaluate()
}

#[test]
fn it_recognizes_overlines_as_draws() {
    assert_eq!(evaluate_at(PieceType::Player1, 0, 14), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 1, 14), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 2, 14), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 3, 14), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 4, 14), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 5, 14), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 6, 14), Evaluation::Draw);
}

#[test]
fn it_recognizes_short_lines_as_draws() {
    assert_eq!(evaluate_at(PieceType::Player1, 0, 1), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 0, 2), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 0, 3), Evaluation::Draw);
    assert_eq!(evaluate_at(PieceType::Player1, 0, 4), Evaluation::Draw);
}

#[test]
fn it_recognizes_horizontal_black_wins() {
    assert_eq!(evaluate_at(PieceType::Player1, 5, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 6, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 7, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 8, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 9, 3), Evaluation::WinPlayer1);
}

#[test]
fn it_recognizes_vertical_black_wins() {
    assert_eq!(evaluate_at(PieceType::Player1, 9, 2), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 9, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 9, 4), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 9, 5), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 9, 6), Evaluation::WinPlayer1);
}

#[test]
fn it_recognizes_diagonal_black_wins() {
    assert_eq!(evaluate_at(PieceType::Player1, 9, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 10, 4), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 11, 5), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 12, 6), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 13, 7), Evaluation::WinPlayer1);
}

#[test]
fn it_recognizes_anti_diagonal_black_wins() {
    assert_eq!(evaluate_at(PieceType::Player1, 8, 4), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 9, 3), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 10, 2), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 11, 1), Evaluation::WinPlayer1);
    assert_eq!(evaluate_at(PieceType::Player1, 12, 0), Evaluation::WinPlayer1);
}

#[test]
fn it_recognizes_diagonal_white_wins() {
    assert_eq!(evaluate_at(PieceType::Player2, 0, 0), Evaluation::WinPlayer2);
    assert_eq!(evaluate_at(PieceType::Player2, 1, 1), Evaluation::WinPlayer2);
    assert_eq!(evaluate_at(PieceType::Player2, 2, 2), Evaluation::WinPlayer2);
    assert_eq!(evaluate_at(PieceType::Player2, 3, 3), Evaluation::WinPlayer2);
    assert_eq!(evaluate_at(PieceType::Player2, 4, 4), Evaluation::WinPlayer2);
}
