use crate::squiggle::Data;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    x: i128,
    y: i128,
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
