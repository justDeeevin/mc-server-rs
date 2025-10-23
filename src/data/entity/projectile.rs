use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::data::{Identifier, Item, block::Block, entity::McUuid};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Projectile {
    has_been_shot: bool,
    #[serde(default)]
    left_owner: bool,
    owner: Option<McUuid>,
    #[serde(flatten)]
    unique: Option<UniqueProjectile>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UniqueProjectile {
    Arrow(Arrow),
    BreezeWindCharge(Hurting),
    DragonFireball(Hurting),
    Egg(RenderAs),
    EnderPearl(RenderAs),
    ExperienceBottle(RenderAs),
    #[serde(rename_all = "PascalCase")]
    Fireball {
        explosion_power: i8,
        item: Option<Item>,
    },
    #[serde(rename_all = "PascalCase")]
    FireworkRocket {
        fireworks_item: Item,
        life: i32,
        life_time: i32,
        shot_at_angle: bool,
    },
    LingeringPotion(RenderAs),
    #[serde(rename_all = "PascalCase")]
    ShulkerBullet {
        steps: i32,
        target: McUuid,
        t_x_d: f64,
        t_y_d: f64,
        t_z_d: f64,
    },
    SmallFireball {
        acceleration_power: f64,
        #[serde(rename = "Item")]
        item: Option<Item>,
    },
    Snowball(RenderAs),
    #[serde(rename_all = "PascalCase")]
    SpectralArrow {
        #[serde(flatten)]
        arrow: Arrow,
        duration: i32,
    },
    SplashPotion(RenderAs),
    #[serde(rename_all = "PascalCase")]
    Trident {
        #[serde(flatten)]
        // Boxed at recommendation of compiler
        arrow: Box<Arrow>,
        dealt_damage: bool,
        #[serde(rename = "item")]
        item: Item,
    },
    WindCharge(Hurting),
    WitherSkull {
        acceleration_power: f64,
        dangerous: bool,
    },
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Arrow {
    crit: bool,
    damage: f64,
    in_block_state: Option<Block>,
    in_groud: bool,
    life: i16,
    pickup: PickupGamemode,
    #[serde(rename = "PierceLevel")]
    pierce_level: i8,
    shake: i8,
    #[serde(rename = "ShotFromCrossbow")]
    shot_from_crossbow: bool,
    #[serde(rename = "SoundEvent")]
    sount_event: Option<Identifier>,
    item: Item,
    weapon: Item,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i8", into = "i8")]
pub enum PickupGamemode {
    None,
    Both,
    Creative,
}

impl TryFrom<i8> for PickupGamemode {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid pickup gamemode")
    }
}

impl From<PickupGamemode> for i8 {
    fn from(value: PickupGamemode) -> Self {
        value as i8
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Hurting {
    acceleration_power: f64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RenderAs {
    item: Option<Item>,
}
