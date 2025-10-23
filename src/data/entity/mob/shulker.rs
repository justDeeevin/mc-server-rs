use serde::{Deserialize, Serialize};

use crate::{
    OneOf,
    data::{block::Face, entity::mob::Color},
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Shulker {
    attach_face: Face,
    color: OneOf<Color, ShulkerPurple>,
    peek: i8,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(into = "i8", try_from = "i8")]
pub struct ShulkerPurple;

impl TryFrom<i8> for ShulkerPurple {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 16 {
            Ok(Self)
        } else {
            Err("Invalid extra shulker color")
        }
    }
}

impl From<ShulkerPurple> for i8 {
    fn from(_value: ShulkerPurple) -> Self {
        16
    }
}
