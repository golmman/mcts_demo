use crate::common::point2d::Point2D;
use crate::common::tree::Tree;
use crate::common::{BOARD_WIDTH, BOARD_SIZE};
use crate::movegen::Move;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Player1 = 0,
    Player2 = 1,
}

#[derive(Default)]
pub struct PlayoutData {
    wins: i64,
    total: i64,
}

pub type Board = [Option<PieceType>; BOARD_SIZE];

pub struct State {
    pub board: Board,
    pub moves: Vec<Move>,
    pub tree: Tree<PlayoutData>,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE],
            moves: Vec::new(),
            tree: Tree::new(),
        }
    }

    pub fn is_blacks_turn(&self) -> bool {
        if self.moves.len() % 2 == 0 {
            return true;
        }

        return false;
    }

    pub fn get_piece_at(&self, p: &Point2D<i8>) -> Option<&PieceType> {
        self.board[BOARD_WIDTH * p.get_y() as usize + p.get_x() as usize].as_ref()
    }

    pub fn make_move(&mut self, mov: Move) {
        if self.moves.len() % 2 == 0 {
            self.board[usize::from(mov)] = Some(PieceType::Player1);
        } else {
            self.board[usize::from(mov)] = Some(PieceType::Player2);
        }

        self.moves.push(mov);
    }
}
