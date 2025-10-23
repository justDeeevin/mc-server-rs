use super::Angerable;
use crate::data::Identifier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enderman {
    carried_block_state: Option<CarriedBlock>,
    #[serde(flatten)]
    angerable: Angerable,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CarriedBlock {
    name: Identifier,
    // TODO: how is this actually represented
    properties: HashMap<String, String>,
}
