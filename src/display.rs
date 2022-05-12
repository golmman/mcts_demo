use std::fmt::Display;

use crate::common::BOARD_SIZE;
use crate::state::{PieceType, State};

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                board.push(' ');
                match self.get_piece_at(x, y) {
                    Some(PieceType::Black) => board.push('X'),
                    Some(PieceType::White) => board.push('O'),
                    None => board.push('·'),
                }
            }
            board.push('\n');
        }
        write!(f, "{}", board)
    }
}