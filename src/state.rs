use crate::common::BOARD_SIZE;
use crate::movegen::Move;

#[derive(Clone, PartialEq)]
pub enum PieceType {
    Black,
    White,
}

pub struct State {
    pub board: Vec<Option<PieceType>>,
    pub moves: Vec<Move>,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: vec![None; BOARD_SIZE * BOARD_SIZE],
            moves: Vec::new(),
        }
    }

    pub fn is_blacks_turn(&self) -> bool {
        if self.moves.len() % 2 == 0 {
            return true;
        }

        return false;
    }

    pub fn get_piece_at(&self, x: usize, y: usize) -> Option<&PieceType> {
        self.board[BOARD_SIZE * y + x].as_ref()
    }

    pub fn make_move(&mut self, mov: Move) {
        if self.moves.len() % 2 == 0 {
            self.board[mov as usize] = Some(PieceType::Black);
        } else {
            self.board[mov as usize] = Some(PieceType::White);
        }

        self.moves.push(mov);
    }
}
