use crate::{
    OneOf,
    data::{Identifier, entity::mob::Breedable},
};
use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

#[derive(Serialize, Deserialize, Clone)]
pub struct Frog {
    variant: OneOf<Variant, Identifier>,
    #[serde(flatten)]
    breedable: Breedable,
}

#[derive(Serialize, Deserialize, Clone, IntoStaticStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "Identifier", into = "Identifier")]
pub enum Variant {
    Temperate,
    Warm,
    Cold,
}

impl TryFrom<Identifier> for Variant {
    type Error = &'static str;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        value.path.parse().map_err(|_| "Invalid variant")
    }
}

impl From<Variant> for Identifier {
    fn from(value: Variant) -> Self {
        Self {
            namespace: "minecraft".to_string(),
            path: <&'static str>::from(value).to_string(),
        }
    }
}
