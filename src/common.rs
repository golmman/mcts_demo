use self::point2d::Point2D;

pub mod point2d;
pub mod rand;
pub mod tree;

#[cfg(test)]
mod tree_test;

pub const BOARD_WIDTH: usize = 15;
pub const BOARD_HEIGHT: usize = 15;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub const DIRECTION_N: Point2D<i8> = Point2D(0, -1);
pub const DIRECTION_E: Point2D<i8> = Point2D(1, 0);
pub const DIRECTION_S: Point2D<i8> = Point2D(0, 1);
pub const DIRECTION_W: Point2D<i8> = Point2D(-1, 0);
pub const DIRECTION_NE: Point2D<i8> = Point2D(1, -1);
pub const DIRECTION_SE: Point2D<i8> = Point2D(1, 1);
pub const DIRECTION_SW: Point2D<i8> = Point2D(-1, 1);
pub const DIRECTION_NW: Point2D<i8> = Point2D(-1, -1);

pub fn is_valid_coord(p: &Point2D<i8>) -> bool {
    if p.get_x() < 0 || p.get_x() >= BOARD_WIDTH as i8 {
        return false;
    }

    if p.get_y() < 0 || p.get_y() >= BOARD_HEIGHT as i8 {
        return false;
    }

    return true;
}
