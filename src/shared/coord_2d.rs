use std::ops;

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub struct Coord2D {
    x: i32,
    y: i32,
}

pub const ORIGIN: Coord2D = Coord2D { x: 0, y: 0 };
pub const LEFT: Coord2D = Coord2D { x: -1, y: 0 };
pub const RIGHT: Coord2D = Coord2D { x: 1, y: 0 };
pub const UP: Coord2D = Coord2D { x: 0, y: 1 };
pub const DOWN: Coord2D = Coord2D { x: 0, y: -1 };
pub const LEFT_UP: Coord2D = Coord2D { x: -1, y: 1 };
pub const RIGHT_UP: Coord2D = Coord2D { x: 1, y: 1 };
pub const LEFT_DOWN: Coord2D = Coord2D { x: -1, y: -1 };
pub const RIGHT_DOWN: Coord2D = Coord2D { x: 1, y: -1};

pub const DIRECTIONS: [&Coord2D; 4] = [&LEFT, &RIGHT, &UP, &DOWN];
pub const ALL_DIRECTIONS: [&Coord2D; 8] = [&LEFT, &RIGHT, &UP, &DOWN, &LEFT_UP, &RIGHT_UP, &LEFT_DOWN, &RIGHT_DOWN];


impl Coord2D {
    pub fn new(x: i32, y: i32) -> Coord2D {
        Coord2D {x, y}
    }

    pub fn manhattan_distance(self, to: &Coord2D) -> i32 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

impl ops::Add<&Coord2D> for Coord2D {
    type Output = Coord2D;

    fn add(self, rhs: &Coord2D) -> Coord2D { &self + rhs }
}

impl ops::Add<&Coord2D> for &Coord2D {
    type Output = Coord2D;

    fn add(self, rhs: &Coord2D) -> Coord2D {
        Coord2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Mul<i32> for &Coord2D {
    type Output = Coord2D;

    fn mul(self, rhs: i32) -> Coord2D {
        Coord2D {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_coords() {
        let coord_1 = Coord2D::new(10, 25);
        let coord_2 = Coord2D::new(-5, 2);
        let expected = Coord2D::new(5, 27);
        assert_eq!(coord_1 + &coord_2, expected);
        assert_eq!(&coord_1 + &coord_2, expected);
    }

    #[test]
    fn can_calculate_manhattan_distance() {
        assert_eq!(Coord2D::new(1, 1).manhattan_distance(&Coord2D::new(0, 0)), 2);
        assert_eq!(Coord2D::new(2, 1).manhattan_distance(&Coord2D::new(0, 0)), 3);
        assert_eq!(Coord2D::new(-3, -2).manhattan_distance(&Coord2D::new(1, 2)), 8);
    }

    #[test]
    fn directions_has_correct_values() {
        assert!(DIRECTIONS.contains(&&Coord2D::new(1, 0)));
        assert!(DIRECTIONS.contains(&&Coord2D::new(0, 1)));
        assert!(DIRECTIONS.contains(&&Coord2D::new(-1, 0)));
        assert!(DIRECTIONS.contains(&&Coord2D::new(0, -1)));
    }
}