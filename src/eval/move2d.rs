use crate::common::BOARD_WIDTH;
use crate::movegen::Move;

#[derive(Clone)]
pub struct Move2D {
    pub x: i8,
    pub y: i8,
}

impl From<Move> for Move2D {
    fn from(m: Move) -> Self {
        Self {
            x: (m % BOARD_WIDTH as u8) as i8,
            y: (m / BOARD_WIDTH as u8) as i8,
        }
    }
}

pub const DIRECTION_N: Move2D = Move2D { x: 0, y: -1 };
pub const DIRECTION_E: Move2D = Move2D { x: 1, y: 0 };
pub const DIRECTION_S: Move2D = Move2D { x: 0, y: 1 };
pub const DIRECTION_W: Move2D = Move2D { x: -1, y: 0 };
pub const DIRECTION_NE: Move2D = Move2D { x: 1, y: -1 };
pub const DIRECTION_SE: Move2D = Move2D { x: 1, y: 1 };
pub const DIRECTION_SW: Move2D = Move2D { x: -1, y: 1 };
pub const DIRECTION_NW: Move2D = Move2D { x: -1, y: -1 };
