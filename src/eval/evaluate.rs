use crate::common::{is_valid_coord, BOARD_WIDTH};
use crate::movegen::Move;
use crate::state::{PieceType, State};

use super::move2d::{
    Move2D, DIRECTION_E, DIRECTION_N, DIRECTION_NE, DIRECTION_NW, DIRECTION_S, DIRECTION_SE,
    DIRECTION_SW, DIRECTION_W,
};
use super::Eval;

impl Eval for State {
    fn evaluate(&self, last_move: Move) -> f32 {
        let (piece_type, score) = if self.is_blacks_turn() {
            (PieceType::Black, 1.0)
        } else {
            (PieceType::White, -1.0)
        };

        let start = Move2D::from(last_move);

        let directions = vec![
            (&DIRECTION_N, &DIRECTION_S),
            (&DIRECTION_W, &DIRECTION_E),
            (&DIRECTION_NW, &DIRECTION_SE),
            (&DIRECTION_NE, &DIRECTION_SW),
        ];

        for direction in directions {
            if 5 == 1
                + count_direction(&self, &piece_type, &start, direction.0)
                + count_direction(&self, &piece_type, &start, direction.1)
            {
                return score;
            }
        }

        0.0
    }
}

fn count_direction(
    state: &State,
    piece_type: &PieceType,
    start: &Move2D,
    direction: &Move2D,
) -> u8 {
    let mut count = 0;
    let mut pos = start.clone();

    loop {
        pos.x += direction.x;
        pos.y += direction.y;

        if !is_valid_coord(pos.x, pos.y) {
            break;
        }

        if state.get_piece_at(pos.x as usize, pos.y as usize) != Some(piece_type) {
            break;
        }

        count += 1;
    }

    count
}
