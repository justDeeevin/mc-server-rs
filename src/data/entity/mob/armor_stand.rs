use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ArmorStand {
    disabled_slots: ArmorStandSlots,
    invisible: bool,
    marker: Option<bool>,
    no_base_plate: bool,
    pose: ArmorStandPose,
    show_arms: bool,
    small: bool,
}

bitflags! {
    #[derive(Serialize, Deserialize, Clone)]
    #[serde(transparent)]
    pub struct ArmorStandSlots: i32 {
        const ADD_CHANGE_MAINHAND = 1 << 0;
        const ADD_CHANGE_BOOTS = 1 << 1;
        const ADD_CHANGE_LEGGINGS = 1 << 2;
        const ADD_CHANGE_CHESTPLATE = 1 << 3;
        const ADD_CHANGE_HELMET = 1 << 4;
        const ADD_CHANGE_OFFHAND = 1 << 5;
        const REMOVE_CHANGE_MAINHAND = 1 << 8;
        const REMOVE_CHANGE_BOOTS = 1 << 9;
        const REMOVE_CHANGE_LEGGINGS = 1 << 10;
        const REMOVE_CHANGE_CHESTPLATE = 1 << 11;
        const REMOVE_CHANGE_HELMET = 1 << 12;
        const REMOVE_CHANGE_OFFHAND = 1 << 13;
        const ADD_MAINHAND = 1 << 16;
        const ADD_BOOTS = 1 << 17;
        const ADD_LEGGINGS = 1 << 18;
        const ADD_CHESTPLATE = 1 << 19;
        const ADD_HELMET = 1 << 20;
        const ADD_OFFHAND = 1 << 21;
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ArmorStandPose {
    body: [f32; 3],
    head: [f32; 3],
    left_arm: [f32; 3],
    left_leg: [f32; 3],
    right_arm: [f32; 3],
    right_leg: [f32; 3],
}
