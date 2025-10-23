use crate::data::{Identifier, entity::mob::Breedable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Mooshroom {
    #[serde(default)]
    stew_effects: Vec<StewEffect>,
    #[serde(rename = "Type")]
    kind: Identifier,
    #[serde(flatten)]
    breedable: Breedable,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StewEffect {
    id: Option<Identifier>,
    duration: Option<i32>,
}
