use serde::{Deserialize, Serialize};

use crate::data::entity::{McUuid, mob::allay::VibrationEvent};

#[derive(Serialize, Deserialize, Clone)]
pub struct Warden {
    #[serde(default)]
    anger: Anger,
    listener: Listener,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Anger {
    #[serde(default)]
    suspects: Vec<Suspect>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Suspect {
    anger: i32,
    uuid: McUuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Listener {
    event: VibrationEvent,
    event_delay: i32,
    selector: Selector,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Selector {
    tick: i64,
    event: VibrationEvent,
}
