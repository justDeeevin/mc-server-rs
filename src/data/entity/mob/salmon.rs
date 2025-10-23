use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Salmon {
    from_bucket: bool,
    #[serde(rename = "type")]
    size: SalmonSize,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SalmonSize {
    Small,
    Medium,
    Large,
}
