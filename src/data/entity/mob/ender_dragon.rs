use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EnderDragon {
    dragon_phase: DragonPhase,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(into = "i32", try_from = "i32")]
pub enum DragonPhase {
    Circling,
    Strafing,
    FlyingToLand,
    Landing,
    TakingOff,
    Breathing,
    LookingForPlayer,
    Roaring,
    Charging,
    FlyingToDie,
    Hovering,
}

impl TryFrom<i32> for DragonPhase {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid dragon phase")
    }
}

impl From<DragonPhase> for i32 {
    fn from(value: DragonPhase) -> Self {
        value as i32
    }
}
