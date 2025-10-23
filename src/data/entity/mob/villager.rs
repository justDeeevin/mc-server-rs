use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::data::{Identifier, Item, entity::McUuid};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Villager {
    #[serde(default)]
    gossips: Vec<Gossip>,
    offers: Offers,
    villager_data: VillagerData,
    xp: i32,
    #[serde(default)]
    inventory: Vec<Item>,
    last_restock: i64,
    last_gossip_decay: i64,
    restocks_today: i32,
    willing: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Gossip {
    value: i32,
    target: McUuid,
    #[serde(rename = "Type")]
    kind: GossipKind,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum GossipKind {
    MajorNegative,
    MinorNegative,
    MajorPositive,
    MinorPositive,
    Trading,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Offers {
    #[serde(default)]
    recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    buy: Item,
    buy_b: Item,
    demand: i32,
    max_uses: i32,
    price_multiplier: f32,
    reward_exp: bool,
    sell: Item,
    special_price: i32,
    uses: i32,
    xp: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VillagerData {
    level: Level,
    // TODO: enum for these two?
    profession: Identifier,
    #[serde(rename = "type")]
    kind: Identifier,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(try_from = "i32", into = "i32")]
pub enum Level {
    Novice = 1,
    Apprentice,
    Journeyman,
    Expert,
    Master,
}

impl TryFrom<i32> for Level {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Level::from_repr(value as usize).ok_or("Invalid level")
    }
}

impl From<Level> for i32 {
    fn from(value: Level) -> Self {
        value as i32
    }
}
