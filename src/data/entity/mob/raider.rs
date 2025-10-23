use crate::data::entity::McUuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Raider {
    can_join_raid: bool,
    patrol_leader: bool,
    pratrolling: bool,
    #[serde(rename = "patrol_target")]
    patrol_target: Option<McUuid>,
    raid_id: i32,
    wave: i32,
    spell_ticks: i32,
}
