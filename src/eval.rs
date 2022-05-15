use crate::common::{BOARD_SIZE, is_valid_coord};
use crate::movegen::Move;
use crate::state::{PieceType, State};

trait Eval {
    fn evaluate(&self, last_move: Move) -> f32;
}

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

#[derive(Clone)]
struct Move2D {
    x: i8,
    y: i8,
}

impl From<Move> for Move2D {
    fn from(m: Move) -> Self {
        Self {
            x: (m % BOARD_SIZE as u8) as i8,
            y: (m / BOARD_SIZE as u8) as i8,
        }
    }
}

const DIRECTION_N: Move2D = Move2D { x: 0, y: -1 };
const DIRECTION_E: Move2D = Move2D { x: 1, y: 0 };
const DIRECTION_S: Move2D = Move2D { x: 0, y: 1 };
const DIRECTION_W: Move2D = Move2D { x: -1, y: 0 };

const DIRECTION_NE: Move2D = Move2D { x: 1, y: -1 };
const DIRECTION_SE: Move2D = Move2D { x: 1, y: 1 };
const DIRECTION_SW: Move2D = Move2D { x: -1, y: 1 };
const DIRECTION_NW: Move2D = Move2D { x: -1, y: -1 };

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
