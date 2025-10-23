use crate::{
    OneOf,
    data::{
        Identifier, Item, ItemWithSlot,
        block::{Block, BlockEntity},
        entity::Entity,
    },
    text_component::TextComponent,
};
use serde::{Deserialize, Serialize};
use serde_with::{FromInto, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct MinecraftComponents {
    #[serde(default)]
    attribute_modifiers: Vec<AttributeModifier>,
    #[serde(default)]
    banner_patterns: Vec<BannerPattern>,
    base_color: Option<String>,
    #[serde(default)]
    bees: Vec<Bee>,
    // FIXME: excludes x, y, z, id, components, and keepPacked
    block_entity_data: Option<BlockEntity>,
    block_state: Option<Block>,
    bucket_entity_data: Option<BucketEntity>,
    bundle_contents: Vec<Item>,
    #[serde(default)]
    #[serde_as(as = "FromInto<OneOf<BlockPredicate, Vec<BlockPredicate>>>")]
    can_break: Vec<BlockPredicate>,
    #[serde(default)]
    #[serde_as(as = "FromInto<OneOf<BlockPredicate, Vec<BlockPredicate>>>")]
    can_place_on: Vec<BlockPredicate>,
    #[serde(default)]
    charged_projectiles: Vec<Item>,
    consumable: Option<Consumable>,
    #[serde(default)]
    container: Vec<ItemWithSlot<i32>>,
    container_loot: Option<Loot>,
    // FIXME
    custom_data: Option<fastnbt::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Loot {
    loot_table: Identifier,
    seed: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Consumable {
    #[serde(default = "default_consume_seconds")]
    consume_seconds: f32,
    #[serde(default)]
    animation: ConsumableAnimation,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConsumableAnimation {
    None,
    #[default]
    Eat,
    Drink,
    Block,
    Bow,
    Spear,
    Crossbow,
    Spyglass,
    TootHorn,
    Brush,
    Bundle,
}

const fn default_consume_seconds() -> f32 {
    1.6
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct BlockPredicate {
    #[serde_as(as = "FromInto<OneOf<Identifier, Vec<Identifier>>>")]
    blocks: Vec<Identifier>,
    nbt: BlockEntity,
    state: Block,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BucketEntity {
    #[serde(rename = "NoAI")]
    no_ai: bool,
    silent: bool,
    no_gravity: bool,
    glowing: bool,
    invulnerable: bool,
    health: f32,
    age: i32,
    variant: i32,
    hunting_cooldown: i64,
    bucket_variant_tag: i32,
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bee {
    entity_data: Entity,
    min_ticks_in_hive: i32,
    ticks_in_hive: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AttributeModifier {
    // TODO: is this to be an enum?
    #[serde(rename = "type")]
    kind: Identifier,
    #[serde(default)]
    slot: ItemSlot,
    id: Identifier,
    amount: f64,
    operation: AttributeModifierOperation,
    display: Option<Display>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BannerPattern {
    color: String,
    pattern: OneOf<BannerPatternInner, Identifier>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BannerPatternInner {
    asset_id: Identifier,
    translation: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Display {
    Default,
    Hidden,
    // Boxed at recommendation of compiler
    Override { value: Box<TextComponent> },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayKind {
    Default,
    Hidden,
    Override,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AttributeModifierOperation {
    AddValue,
    AddMultipliedBase,
    AddMultipliedTotal,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ItemSlot {
    #[default]
    Any,
    Hand,
    Armor,
    MainHand,
    Offhand,
    Head,
    Chest,
    Legs,
    Feet,
    Body,
    Saddle,
}
