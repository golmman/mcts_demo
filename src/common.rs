pub const BOARD_WIDTH: usize = 15;
pub const BOARD_HEIGHT: usize = 15;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

pub fn is_valid_coord(x: i8, y: i8) -> bool {
    if x <= 0 || x > BOARD_WIDTH as i8 {
        return false;
    }

    if y <= 0 || y > BOARD_HEIGHT as i8 {
        return false;
    }

    return true;
}
