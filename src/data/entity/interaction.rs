use serde::{Deserialize, Serialize};

use super::{McUuid, one_f32};

#[derive(Serialize, Deserialize, Clone)]
pub struct Interaction {
    #[serde(default = "one_f32")]
    width: f32,
    #[serde(default = "one_f32")]
    height: f32,
    #[serde(default)]
    response: bool,
    attack: Hit,
    interaction: Hit,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hit {
    player: McUuid,
    timestamp: i64,
}
