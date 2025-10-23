use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Zombie {
    can_break_doors: bool,
    drowned_conversion_time: i32,
    in_water_time: i32,
    is_baby: Option<bool>,
}
