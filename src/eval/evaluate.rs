use crate::common::is_valid_coord;
use crate::common::point2d::Point2D;
use crate::movegen::Move;
use crate::state::{PieceType, State};

use super::Eval;
use crate::common::{
    DIRECTION_E, DIRECTION_N, DIRECTION_NE, DIRECTION_NW, DIRECTION_S, DIRECTION_SE, DIRECTION_SW,
    DIRECTION_W,
};

impl Eval for State {
    fn evaluate(&self, last_move: Move) -> f32 {
        let (piece_type, score) = if self.is_blacks_turn() {
            (PieceType::Black, 1.0)
        } else {
            (PieceType::White, -1.0)
        };

        let start = Point2D::from(last_move);

        let directions = vec![
            (DIRECTION_N, DIRECTION_S),
            (DIRECTION_W, DIRECTION_E),
            (DIRECTION_NW, DIRECTION_SE),
            (DIRECTION_NE, DIRECTION_SW),
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
    start: &Point2D<i8>,
    direction: Point2D<i8>,
) -> u8 {
    let mut count = 0;
    let mut pos = start.clone();

    loop {
        pos += direction.clone();

        if !is_valid_coord(&pos) {
            break;
        }

        if state.get_piece_at(&pos) != Some(piece_type) {
            break;
        }

        count += 1;
    }

    count
}
