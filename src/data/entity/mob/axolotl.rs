use super::Breedable;
use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
pub struct Axolotl {
    from_bucket: bool,
    variant: Variant,
    #[serde(flatten)]
    breedable: Breedable,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i32", into = "i32")]
pub enum Variant {
    Lucy,
    Wind,
    Gold,
    Cyan,
    Blue,
}

impl TryFrom<i32> for Variant {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid variant")
    }
}

impl From<Variant> for i32 {
    fn from(value: Variant) -> Self {
        value as i32
    }
}
