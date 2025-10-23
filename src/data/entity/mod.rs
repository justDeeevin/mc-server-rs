pub mod area_effect_cloud;
pub mod display;
pub mod hanging;
pub mod interaction;
pub mod mob;
pub mod projectile;
pub mod vehicle;

use crate::{
    data::{
        Identifier, Item,
        block::{Block, BlockEntity},
        entity::{
            area_effect_cloud::AreaEffectCloud,
            display::Display,
            hanging::Hanging,
            interaction::Interaction,
            mob::Mob,
            projectile::{Projectile, RenderAs},
            vehicle::Vehicle,
        },
    },
    text_component::TextComponent,
};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Entity {
    air: Option<i16>,
    custom_name: Option<TextComponent>,
    #[serde(default)]
    custom_name_visible: bool,
    #[serde(rename = "fall_distance")]
    fall_distance: f64,
    fire: i16,
    #[serde(default)]
    glowing: bool,
    has_visual_fire: bool,
    #[serde(rename = "id")]
    id: Option<Identifier>,
    invulnerable: bool,
    motion: Motion,
    no_gravity: bool,
    on_ground: bool,
    #[serde(default)]
    passengers: Vec<Self>,
    portal_cooldown: i32,
    pos: [f64; 3],
    rotation: Rotation,
    silent: Option<bool>,
    #[serde(default)]
    tags: Vec<String>,
    ticks_frozen: i32,
    #[serde(rename = "UUID")]
    uuid: McUuid,
    #[serde(flatten)]
    subtype: Option<EntitySubtype>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
// Boxes from compiler recommendation
pub enum EntitySubtype {
    Mob(Box<Mob>),
    Projectile(Box<Projectile>),
    #[serde(rename_all = "PascalCase")]
    ExperienceOrb {
        age: i16,
        count: i32,
        health: i16,
        value: i16,
    },
    #[serde(rename_all = "PascalCase")]
    Item {
        age: i16,
        health: i16,
        item: Box<Item>,
        owner: Option<McUuid>,
        pickup_delay: i16,
        thrower: McUuid,
    },
    Vehicle(Vehicle),
    #[serde(rename_all = "PascalCase")]
    FallingBlock {
        block_state: Block,
        cancel_drop: bool,
        drop_item: bool,
        fall_hurt_amount: f32,
        fall_hurt_max: i32,
        hurt_enemies: bool,
        tile_entity_data: Option<BlockEntity>,
        time: i32,
    },
    Tnt {
        #[serde(default = "default_tnt_fuse")]
        fuse: i16,
        #[serde(default = "tnt_block")]
        block_state: Block,
        #[serde(default = "default_tnt_explotion_power")]
        explosion_power: f32,
        owner: Option<McUuid>,
    },
    // Boxed at recommendation of compiler
    Display(Box<Display>),
    AreaEffectCloud(AreaEffectCloud),
    EndCrystal {
        beam_target: [i32; 3],
        #[serde(rename = "ShowBottom")]
        show_bottom: bool,
    },
    #[serde(rename_all = "PascalCase")]
    EvokerFangs {
        owner: McUuid,
        warmup: i32,
    },
    EyeOfEnder(RenderAs),
    Hanging(Hanging),
    Interaction(Interaction),
    OminousItemSpawner {
        item: Item,
        spawn_item_after_ticks: i64,
    },
}

const fn default_tnt_fuse() -> i16 {
    80
}

const fn default_tnt_explotion_power() -> f32 {
    4.0
}

fn tnt_block() -> Block {
    todo!("TNT block value")
}

#[derive(Clone, Deref, DerefMut, AsRef, AsMut)]
pub struct McUuid(Uuid);

impl Serialize for McUuid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;

        let bytes = self.as_bytes();
        let mut seq = serializer.serialize_seq(Some(4))?;
        for bytes in bytes.chunks(4) {
            seq.serialize_element(&i32::from_be_bytes(bytes.try_into().unwrap()))?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for McUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let ints = <[i32; 4]>::deserialize(deserializer)?;
        let mut bytes = [0; 16];
        for (i, int) in ints.iter().enumerate() {
            bytes[i * 4..(i + 1) * 4].copy_from_slice(&int.to_be_bytes());
        }
        Ok(Self(Uuid::from_bytes(bytes)))
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(from = "[f64; 3]", into = "[f64; 3]")]
pub struct Motion {
    x: f64,
    y: f64,
    z: f64,
}

impl From<[f64; 3]> for Motion {
    fn from(value: [f64; 3]) -> Self {
        match value {
            [-10.0..=10.0, -10.0..=10.0, -10.0..=10.0] => Self {
                x: value[0],
                y: value[1],
                z: value[2],
            },
            _ => Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

impl From<Motion> for [f64; 3] {
    fn from(value: Motion) -> Self {
        [value.x, value.y, value.z]
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(from = "[f32; 2]", into = "[f32; 2]")]
pub struct Rotation {
    yaw: f32,
    pitch: f32,
}

impl From<[f32; 2]> for Rotation {
    fn from(value: [f32; 2]) -> Self {
        let yaw = {
            let mut yaw = value[0].rem_euclid(360.0);
            if yaw > 180.0 {
                yaw -= 360.0;
            }

            yaw
        };
        Self {
            yaw,
            pitch: value[1].clamp(-90.0, 90.0),
        }
    }
}

impl From<Rotation> for [f32; 2] {
    fn from(value: Rotation) -> Self {
        [value.yaw, value.pitch]
    }
}

const fn one_f32() -> f32 {
    1.0
}
