use crate::squiggle::Data;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: i128,
    pub y: i128,
}

impl From<Vec2> for Data {
    fn from(v: Vec2) -> Data {
        Data::make_cons(v.x, v.y)
    }
}

impl TryFrom<Data> for Vec2 {
    type Error = String;

    fn try_from(data: Data) -> Result<Self, Self::Error> {
        let (x, y) = data.try_to_coords().ok_or("not a pair of numbers")?;
        Ok(Vec2 { x, y })
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Vec2 {
    pub fn norm(self) -> i128 {
        self.x.abs().max(self.y.abs())
    }
}
