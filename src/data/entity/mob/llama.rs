use super::{ChestedHorse, Horse};
use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Llama {
    #[serde(flatten)]
    horse: Horse,
    #[serde(flatten)]
    chested_horse: ChestedHorse,
    strength: i32,
    variant: Variant,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i32", into = "i32")]
pub enum Variant {
    Creamy,
    White,
    Brown,
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
