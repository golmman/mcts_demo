use std::ops::{Add, AddAssign};

#[derive(Clone)]
pub struct Point2D<T>(pub T, pub T);

impl<T: Copy> Point2D<T> {
    pub fn get_x(&self) -> T {
        self.0
    }

    pub fn get_y(&self) -> T {
        self.1
    }
}

impl<T: Add<Output = T> + Copy> Add<&Point2D<T>> for &Point2D<T> {
    type Output = Point2D<T>;

    fn add(self, rhs: &Point2D<T>) -> Self::Output {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Point2D(self.0 + rhs.0, self.1 + rhs.1);
    }
}
