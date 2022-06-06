use crate::common::point2d::Point2D;
use crate::common::BOARD_WIDTH;
use crate::state::State;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Move(pub u8);

impl From<usize> for Move {
    fn from(m: usize) -> Self {
        Self(m as u8)
    }
}

impl From<Move> for usize {
    fn from(m: Move) -> usize {
        m.0 as usize
    }
}

impl From<Move> for Point2D<i8> {
    fn from(m: Move) -> Self {
        Self(m.get_x() as i8, m.get_y() as i8)
    }
}

impl From<Point2D<i8>> for Move {
    fn from(p: Point2D<i8>) -> Self {
        Self(BOARD_WIDTH as u8 * p.get_y() as u8 + p.get_x() as u8)
    }
}

impl Move {
    pub fn get_x(&self) -> u8 {
        self.0 % BOARD_WIDTH as u8
    }

    pub fn get_y(&self) -> u8 {
        self.0 / BOARD_WIDTH as u8
    }
}

pub trait Movegen {
    fn generate_moves(&self) -> Vec<Move>;
}

impl Movegen for State {
    fn generate_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for i in 0..BOARD_WIDTH * BOARD_WIDTH {
            if self.board[i].is_none() {
                moves.push(Move::from(i));
            }
        }

        moves
    }
}
