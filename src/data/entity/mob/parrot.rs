use serde::{Deserialize, Serialize};
use strum::FromRepr;

use super::Tameable;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Parrot {
    variant: Variant,
    #[serde(flatten)]
    tameable: Tameable,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i32", into = "i32")]
pub enum Variant {
    RedBlue,
    Blue,
    Green,
    YellowBlue,
    Gray,
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
