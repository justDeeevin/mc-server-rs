use crate::{
    data::{
        Identifier,
        entity::{McUuid, mob::PotionEffect},
        particle::Particle,
    },
    text_component::ARGB,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct AreaEffectCloud {
    age: i32,
    color: ARGB,
    #[serde(rename = "potion_contents")]
    potion_contents: PotionContents,
    owner: McUuid,
    #[serde(rename = "custom_particle")]
    custom_particle: Particle,
    potion: Identifier,
    #[serde(rename = "potion_duration_scale")]
    potion_duration_scale: f32,
    radius: f32,
    radius_on_use: f32,
    radius_per_tick: f32,
    reapplication_delay: i32,
    wait_time: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PotionContents {
    potion: Option<Identifier>,
    custom_color: ARGB,
    custom_effects: Vec<PotionEffect>,
}
