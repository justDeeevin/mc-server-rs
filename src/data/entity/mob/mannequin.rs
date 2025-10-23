use serde::{Deserialize, Serialize};

use crate::{
    OneOf,
    data::{
        Identifier,
        player::{Model, Profile, SkinLayer},
    },
    text_component::TextComponent,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Mannequin {
    profile: OneOf<MannequinProfile, Profile>,
    #[serde(default)]
    hidden_layers: Vec<SkinLayer>,
    main_hand: Hand,
    pose: Pose,
    #[serde(default)]
    immovable: bool,
    description: Option<TextComponent>,
    hide_description: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MannequinProfile {
    texture: Identifier,
    cape: Option<Identifier>,
    elytra: Option<Identifier>,
    #[serde(default)]
    model: Model,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Hand {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Pose {
    Standing,
    Crouching,
    Swiming,
    FallFlying,
    Sleeping,
}
