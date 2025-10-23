use super::Breedable;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Rabbit {
    #[serde(flatten)]
    breedable: Breedable,
    rabbit_type: RabbitType,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "Option<i32>", into = "Option<i32>")]
pub enum RabbitType {
    Brown,
    White,
    Black,
    WhiteSplotched,
    Gold,
    Salt,
    // THE KILLER BUNNY >:3
    Evil = 99,
    Toast,
}

impl TryFrom<Option<i32>> for RabbitType {
    type Error = &'static str;

    fn try_from(value: Option<i32>) -> Result<Self, Self::Error> {
        if let Some(value) = value {
            Self::from_repr(value as usize).ok_or("Invalid rabbit type")
        } else {
            Ok(Self::Toast)
        }
    }
}

impl From<RabbitType> for Option<i32> {
    fn from(value: RabbitType) -> Self {
        match value {
            RabbitType::Toast => None,
            other => Some(other as i32),
        }
    }
}
