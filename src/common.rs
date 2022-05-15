pub const BOARD_SIZE: usize = 15;

pub fn is_valid_coord(x: i8, y: i8) -> bool {
    if x <= 0 || x > BOARD_SIZE as i8 {
        return false;
    }

    if y <= 0 || y > BOARD_SIZE as i8 {
        return false;
    }

    return true;
}
