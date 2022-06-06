use crate::common::is_valid_coord;
use crate::common::point2d::Point2D;
use crate::state::{PieceType, State};

use super::{Eval, Evaluation};
use crate::common::{
    DIRECTION_E, DIRECTION_N, DIRECTION_NE, DIRECTION_NW, DIRECTION_S, DIRECTION_SE, DIRECTION_SW,
    DIRECTION_W,
};

impl Eval for State {
    fn evaluate(&self) -> Evaluation {
        if self.moves.len() < 9 {
            return Evaluation::Draw;
        }

        let last_move = self.moves[self.moves.len() - 1];

        // note that when it is player1's turn player2 has made the last move
        // so we evaluate for player2 in this case
        let (piece_type, evaluation) = if self.is_player1_turn() {
            (PieceType::Player2, Evaluation::WinPlayer2)
        } else {
            (PieceType::Player1, Evaluation::WinPlayer1)
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
                + self.count_direction(&piece_type, &start, direction.0)
                + self.count_direction(&piece_type, &start, direction.1)
            {
                return evaluation;
            }
        }

        Evaluation::Draw
    }
}

impl State {
    fn count_direction(
        &self,
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

            if self.get_piece_at(&pos) != Some(piece_type) {
                break;
            }

            count += 1;
        }

        count
    }
}
