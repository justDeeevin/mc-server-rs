use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Pufferfish {
    from_bucket: bool,
    puff_state: PuffState,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i32", into = "i32")]
pub enum PuffState {
    Deflated,
    Halfway,
    PuffedUp,
}

impl TryFrom<i32> for PuffState {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid puff state")
    }
}

impl From<PuffState> for i32 {
    fn from(value: PuffState) -> Self {
        value as i32
    }
}
