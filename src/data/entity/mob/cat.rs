use crate::{
    OneOf,
    data::{
        Identifier, MinecraftNamespaceWrapper,
        entity::mob::{Breedable, Color, Tameable},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Cat {
    #[serde(default = "default_collar_color")]
    collar_color: Color,
    #[serde(rename = "variant")]
    variant: OneOf<MinecraftNamespaceWrapper<Variant>, Identifier>,
    #[serde(flatten)]
    breedable: Breedable,
    #[serde(flatten)]
    tameable: Tameable,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Variant {
    White,
    Black,
    Red,
    Siamese,
    BritishShorthair,
    Calico,
    Persian,
    Ragdoll,
    Tabby,
    AllBlack,
    Jellie,
}

const fn default_collar_color() -> Color {
    Color::Red
}
