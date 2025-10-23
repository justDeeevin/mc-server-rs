use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "Name", content = "Properties")]
pub enum Block {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BlockEntity {}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i8", into = "i8")]
pub enum Face {
    Top,
    Bottom,
    North,
    South,
    West,
    East,
}

impl TryFrom<i8> for Face {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid attach face")
    }
}

impl From<Face> for i8 {
    fn from(value: Face) -> Self {
        value as i8
    }
}
