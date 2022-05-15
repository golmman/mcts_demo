use crate::common::BOARD_SIZE;
use crate::state::State;

pub type Move = u8;

trait Movegen {
    fn generate_moves(&self) -> Vec<Move>;
}

impl Movegen for State {
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for i in 0..BOARD_SIZE * BOARD_SIZE {
            if self.board[i].is_none() {
                moves.push(i as u8);
            }
        }

        moves
    }
}
