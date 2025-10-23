use super::Breedable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Armadillo {
    scute_time: i32,
    state: ArmadilloState,
    #[serde(flatten)]
    breedable: Breedable,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ArmadilloState {
    Idle,
    Scared,
    Unrolling,
}

impl From<&str> for ArmadilloState {
    fn from(value: &str) -> Self {
        match value {
            "idle" => Self::Idle,
            "scared" => Self::Scared,
            "unrolling" => Self::Unrolling,
            _ => Self::Idle,
        }
    }
}

impl From<String> for ArmadilloState {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
