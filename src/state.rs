use crate::common::point2d::Point2D;
use crate::common::rand::Random;
use crate::common::tree::Tree;
use crate::common::{BOARD_SIZE, BOARD_WIDTH};
use crate::movegen::Move;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Player1 = 0,
    Player2 = 1,
}

#[derive(Default, Debug)]
pub struct PlayoutData {
    pub mov: Move,
    pub wins: i64,
    pub total: i64,
}

impl From<Move> for PlayoutData {
    fn from(mov: Move) -> Self {
        Self {
            mov,
            wins: 0,
            total: 0,
        }
    }
}

pub type Board = [Option<PieceType>; BOARD_SIZE];

pub struct State {
    pub board: Board,
    pub moves: Vec<Move>,
    pub rand: Random,
    pub tree: Tree<PlayoutData>,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE],
            moves: Vec::new(),
            rand: Random::new(0),
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

    pub fn unmake_move(&mut self) {
        if let Some(mov) = self.moves.pop() {
            self.board[usize::from(mov)] = None;
        }
    }
}
