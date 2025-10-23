use super::{Identifier, ItemWithSlot, entity::McUuid};
use crate::{
    OneOf,
    data::{
        Item, PositionInDimension,
        entity::{Entity, Rotation},
    },
};
use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    #[serde(rename = "abilities")]
    abilities: Abilities,
    #[serde(rename = "current_explosion_impact_pos")]
    current_explosion_impact_pos: [f64; 3],
    #[serde(rename = "entered_nether_pos")]
    entered_nether_pos: Option<[f64; 3]>,
    #[serde(rename = "ignore_fall_damage_from_current_explosion")]
    ignore_fall_damage_from_current_explosion: bool,
    #[serde(rename = "warden_spawn_tracker")]
    warden_spawn_tracker: WardenSpawnTracker,

    data_version: i32,
    dimension: Identifier,
    #[serde(default)]
    ender_items: Vec<ItemWithSlot>,
    #[serde(default)]
    inventory: Vec<Item>,
    last_death_location: Option<PositionInDimension>,
    root_vehicle: RootVehicle,
    score: i32,
    // FIXME: not saved in player.dat
    selected_item: Item,
    selected_item_slot: i32,
    shoulder_entity_left: Entity,
    shoulder_entity_right: Entity,
    sleep_timer: i16,
    spawn_x: i32,
    spawn_y: i32,
    spawn_z: i32,
    spawn_angle: Rotation,
    #[serde(default = "default_spawn")]
    spawn_dimension: Identifier,
    #[serde(default)]
    spawn_forced: bool,
    xp_level: i32,
    xp_p: f32,
    xp_seed: i32,
    xp_total: i32,

    #[serde(rename = "foodExhaustionLevel")]
    food_exhaustion_level: f32,
    #[serde(rename = "foodLevel")]
    food_level: i32,
    #[serde(rename = "foodSaturationLevel")]
    food_saturation_level: f32,
    #[serde(rename = "foodTickTimer")]
    food_tick_timer: i32,
    #[serde(rename = "playerGameType")]
    player_game_type: GameType,
    #[serde(rename = "previousPlayerGameType")]
    previous_player_game_type: GameType,
    #[serde(rename = "recipeBook")]
    recipe_book: RecipeBook,
    #[serde(rename = "seenCredits")]
    seen_credits: bool,
}

fn default_spawn() -> Identifier {
    "overworld".parse().unwrap()
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WardenSpawnTracker {
    warning_level: i32,
    cooldown_ticks: i32,
    ticks_since_last_warning: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct RootVehicle {
    attach: McUuid,
    entity: Entity,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecipeBook {
    #[serde(default)]
    recipes: Vec<Identifier>,
    #[serde(default)]
    to_be_displayed: Vec<Identifier>,
    is_filtering_craftable: bool,
    is_gui_open: bool,
    is_furnace_filtering_craftable: bool,
    is_furnace_gui_open: bool,
    is_blasting_furnace_filtering_craftable: bool,
    is_blasting_furnace_gui_open: bool,
    is_smoker_filtering_craftable: bool,
    is_smoker_gui_open: bool,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i32", into = "i32")]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl TryFrom<i32> for GameType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid game type")
    }
}

impl From<GameType> for i32 {
    fn from(value: GameType) -> Self {
        value as i32
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    flying: bool,
    fly_speed: f32,
    instabuild: bool,
    invulnerable: bool,
    may_build: bool,
    may_fly: bool,
    walk_speed: f32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "OneOf<ProfileInner, String>")]
pub struct Profile(ProfileInner);

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileInner {
    name: Option<String>,
    id: Option<McUuid>,
    texture: Option<Identifier>,
    cape: Option<Identifier>,
    model: Option<Model>,
    #[serde(default)]
    properties: Vec<PlayerProperty>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Model {
    Wide,
    #[default]
    Slim,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerProperty {
    name: PlayerPropertyKind,
    value: String,
    signature: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PlayerPropertyKind {
    Textures,
}

impl From<OneOf<ProfileInner, String>> for Profile {
    fn from(value: OneOf<ProfileInner, String>) -> Self {
        match value {
            OneOf::Left(profile) => Self(profile),
            OneOf::Right(string) => Self(ProfileInner {
                name: Some(string),
                id: None,
                texture: None,
                cape: None,
                model: None,
                properties: vec![],
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SkinLayer {
    Cape,
    Jacket,
    LeftSleeve,
    RightSleeve,
    LeftPantsLeg,
    RightPantsLeg,
    Hat,
}
