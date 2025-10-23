use serde::{Deserialize, Serialize};

use crate::{
    OneOf,
    data::{Identifier, Item, block::Face, entity::one_f32},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Hanging {
    // TODO: wiki claims this is i32 but this feels more accurate
    block_pos: [i32; 3],
    #[serde(flatten)]
    unique: Option<OneOf<ItemFrame, Painting>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ItemFrame {
    #[serde(rename = "Facing")]
    facing: Face,
    fixed: bool,
    invisible: bool,
    item: Option<Item>,
    #[serde(default = "one_f32")]
    item_drop_chance: f32,
    item_rotation: i8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Painting {
    facing: Face,
    variant: Identifier,
}

