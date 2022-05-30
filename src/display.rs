use std::fmt::Display;

use crate::common::BOARD_WIDTH;
use crate::common::point2d::Point2D;
use crate::state::{PieceType, State};

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for y in 0..BOARD_WIDTH {
            for x in 0..BOARD_WIDTH {
                board.push(' ');
                match self.get_piece_at(&Point2D(x as i8, y as i8)) {
                    Some(PieceType::Player1) => board.push('X'),
                    Some(PieceType::Player2) => board.push('O'),
                    None => board.push('·'),
                }
            }
            board.push('\n');
        }
        write!(f, "{}", board)
    }
}
