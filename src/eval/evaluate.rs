use crate::common::is_valid_coord;
use crate::common::point2d::Point2D;
use crate::state::{PieceType, State};

use super::Eval;
use crate::common::{
    DIRECTION_E, DIRECTION_N, DIRECTION_NE, DIRECTION_NW, DIRECTION_S, DIRECTION_SE, DIRECTION_SW,
    DIRECTION_W,
};

impl Eval for State {
    fn evaluate(&self) -> f32 {
        if self.moves.len() < 9 {
            return 0.0;
        }

        let last_move = self.moves[self.moves.len() - 1];

        // note that when it is blacks turn white has made the last move
        // so we evaluate for white in this case
        let (piece_type, score) = if self.is_blacks_turn() {
            (PieceType::Player2, -1.0)
        } else {
            (PieceType::Player1, 1.0)
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
