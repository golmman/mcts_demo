use crate::common::BOARD_SIZE;
use crate::movegen::Move;

#[derive(Clone)]
pub enum PieceType {
    Black,
    White,
}

pub struct State {
    board: Vec<Option<PieceType>>,
    moves: Vec<Move>,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: vec![None; BOARD_SIZE * BOARD_SIZE],
            moves: Vec::new(),
        }
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
