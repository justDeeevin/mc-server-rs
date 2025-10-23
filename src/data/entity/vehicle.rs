use serde::{Deserialize, Serialize};
use serde_with::{FromInto, serde_as};

use crate::{
    OneOf,
    data::{Identifier, ItemWithSlot, block::Block, entity::Entity},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum Vehicle {
    ChestBoat(Container),
    Minecart(Minecart),
    ChestMinecart {
        #[serde(flatten)]
        container: Container,
        #[serde(flatten)]
        minecart: Minecart,
    },
    #[serde(rename_all = "PascalCase")]
    CommandBlockMinecart {
        #[serde(flatten)]
        minecart: Minecart,
        command: String,
        last_output: String,
        success_count: i32,
        track_output: bool,
    },
    #[serde(rename_all = "PascalCase")]
    FurnaceMinecart {
        #[serde(flatten)]
        minecart: Minecart,
        fuel: i16,
        push_x: f64,
        push_z: f64,
    },
    #[serde(rename_all = "PascalCase")]
    HopperMinecart {
        #[serde(flatten)]
        minecart: Minecart,
        #[serde(flatten)]
        container: Container,
        enabled: bool,
    },
    #[serde(rename_all = "PascalCase")]
    SpawnerMinecart {
        #[serde(flatten)]
        minecart: Minecart,
        delay: i16,
        max_nearby_entities: i16,
        max_spawn_delay: i16,
        min_spawn_delay: i16,
        required_player_range: i16,
        spawn_count: i16,
        // Boxed for indirection
        spawn_data: Box<Entity>,
        spawn_potentials: Vec<SpawnPotential>,
        spawn_range: i16,
    },
    TntMinecart {
        #[serde(flatten)]
        minecart: Minecart,
        fuse: i32,
        explosion_power: f32,
        explosion_speed_factor: f32,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpawnPotential {
    weight: i32,
    data: SpawnData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpawnData {
    entity: Entity,
    #[serde(default)]
    custom_spawn_rules: CustomSpawnRules,
    equipment: Option<SpawnEquipment>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct SpawnEquipment {
    loot_table: Identifier,
    #[serde_as(as = "FromInto<OneOf<[f32; 7], DropChances>>")]
    #[serde(default)]
    slot_drop_chances: DropChances,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct DropChances {
    feet: Option<f32>,
    legs: Option<f32>,
    chest: Option<f32>,
    head: Option<f32>,
    body: Option<f32>,
    mainhand: Option<f32>,
    offhand: Option<f32>,
}

impl From<[f32; 7]> for DropChances {
    fn from(value: [f32; 7]) -> Self {
        Self {
            feet: Some(value[0]),
            legs: Some(value[1]),
            chest: Some(value[2]),
            head: Some(value[3]),
            body: Some(value[4]),
            mainhand: Some(value[5]),
            offhand: Some(value[6]),
        }
    }
}

impl From<OneOf<[f32; 7], DropChances>> for DropChances {
    fn from(value: OneOf<[f32; 7], DropChances>) -> Self {
        match value {
            OneOf::Left(value) => value.into(),
            OneOf::Right(value) => value,
        }
    }
}

impl From<DropChances> for OneOf<[f32; 7], DropChances> {
    fn from(value: DropChances) -> Self {
        Self::Right(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CustomSpawnRules {
    block_light_limit: Option<i32>,
    sky_light_limit: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Container {
    #[serde(default)]
    items: Vec<ItemWithSlot>,
    loot_table: Option<Identifier>,
    loot_table_seed: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Minecart {
    display_offset: Option<i32>,
    display_state: Option<Block>,
}
