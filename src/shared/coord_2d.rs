use std::ops;

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub struct Coord2D {
    x: i32,
    y: i32,
}

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

    fn add(self, rhs: &Coord2D) -> Coord2D {
        Coord2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
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