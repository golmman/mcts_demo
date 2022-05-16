use crate::common::BOARD_WIDTH;
use crate::state::State;

pub type Move = u8;

struct Move1(u8);

impl From<u8> for Move1 {
    fn from(m: u8) -> Self {
        Self(m)
    }
}

trait Movegen {
    fn generate_moves(&self) -> Vec<Move>;
}

impl Movegen for State {
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for i in 0..BOARD_WIDTH * BOARD_WIDTH {
            if self.board[i].is_none() {
                moves.push(i as u8);
            }
        }

        moves
    }
}
