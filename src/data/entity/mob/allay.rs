use crate::data::{Identifier, Item, entity::McUuid};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Allay {
    duplication_cooldown: i64,
    #[serde(default)]
    inventory: Vec<Item>,
    #[serde(rename = "listener")]
    listener: VibrationListener,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VibrationListener {
    distance: i32,
    event: Option<VibrationEvent>,
    event_delay: i32,
    event_distance: i32,
    range: i32,
    source: VibrationSource,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum VibrationSource {
    Block { pos: [i32; 3] },
    Entity { uuid: McUuid, y_offset: f32 },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VibrationEvent {
    distance: i32,
    game_event: Identifier,
    pos: [f64; 3],
    projectile_owner: Option<McUuid>,
    source: Option<McUuid>,
}
