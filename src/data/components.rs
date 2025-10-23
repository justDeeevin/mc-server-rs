use crate::{
    OneOf,
    data::{
        Identifier, Item, ItemWithSlot,
        block::{Block, BlockEntity},
        damage::DamageTypeTag,
        entity::{Entity, tag::EntityTypeTag},
        one_f32,
    },
    text_component::TextComponent,
};
use serde::{Deserialize, Serialize};
use serde_with::{FromInto, serde_as};
use std::collections::HashMap;

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
    // FIXME: can accept an SNBT string
    custom_data: Option<HashMap<String, fastnbt::Value>>,
    // TODO: wtf
    // https://minecraft.wiki/w/Data_component_format?oldid=2897248#custom_model_data
    custom_model_data: Option<CustomModelData>,
    custom_name: Option<TextComponent>,
    #[serde(default)]
    damage: i32,
    damage_resistant: Option<DamageResistance>,
    #[serde(default)]
    debug_stick_state: HashMap<Identifier, String>,
    death_protection: Option<DeathProtection>,
    dyed_color: Option<RGB>,
    enchantable: Option<Enchantable>,
    enchantment_glint_override: Option<bool>,
    #[serde(default)]
    enchantments: Enchantments,
    // Boxed for indirection
    entity_data: Option<Box<Entity>>,
    equippable: Option<Equippable>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Enchantments {
    #[serde(default)]
    levels: HashMap<Identifier, i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Equippable {
    slot: EquipSlot,
    equip_sound: Option<OneOf<Identifier, Sound>>,
    asset_id: Option<Identifier>,
    allowed_entities: Option<OneOf<Identifier, OneOf<Vec<Identifier>, EntityTypeTag>>>,
    // NO equip_on_interact
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EquipSlot {
    Head,
    Chest,
    Legs,
    Feet,
    Body,
    Mainhand,
    Offhand,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enchantable {
    value: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DeathProtection {
    #[serde(default)]
    death_effects: Vec<ConsumeEffect>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ConsumeEffect {
    ApplyEffects {
        #[serde(default)]
        effects: Vec<Effect>,
        #[serde(default = "one_f32")]
        probability: f32,
    },
    RemoveEffects {
        #[serde_as(as = "FromInto<OneOf<Vec<Identifier>, Identifier>>")]
        effects: Vec<Identifier>,
    },
    ClearAllEffects,
    TeleportRandomly {
        #[serde(default = "default_random_teleport_diameter")]
        diameter: f32,
    },
    PlaySound {
        sound: OneOf<Identifier, Sound>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Sound {
    sound_id: Identifier,
    range: Option<f32>,
}

const fn default_random_teleport_diameter() -> f32 {
    16.0
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Effect {
    id: Identifier,
    #[serde(default)]
    amplifier: i8,
    #[serde(default = "default_consume_duration")]
    duration: i32,
    #[serde(default)]
    ambient: bool,
    #[serde(default = "true_value")]
    show_particles: bool,
    #[serde(default = "true_value")]
    show_icon: bool,
}

const fn default_consume_duration() -> i32 {
    1
}

const fn true_value() -> bool {
    true
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageResistance {
    types: DamageTypeTag,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomModelData {
    #[serde(default)]
    floats: Vec<f32>,
    #[serde(default)]
    flags: Vec<bool>,
    #[serde(default)]
    strings: Vec<String>,
    colors: Vec<RGB>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "OneOf<i32, [f32; 3]>", into = "i32")]
pub struct RGB {
    r: f32,
    b: f32,
    g: f32,
}

impl From<OneOf<i32, [f32; 3]>> for RGB {
    fn from(value: OneOf<i32, [f32; 3]>) -> Self {
        match value {
            OneOf::Left(int) => Self {
                r: ((int >> 16) as u8) as f32 / 255.0,
                g: ((int >> 8) as u8) as f32 / 255.0,
                b: (int as u8) as f32 / 255.0,
            },
            OneOf::Right(floats) => Self {
                r: floats[0],
                g: floats[1],
                b: floats[2],
            },
        }
    }
}

impl From<RGB> for i32 {
    fn from(value: RGB) -> Self {
        ((value.r * 255.0) as i32) << 16
            | ((value.g * 255.0) as i32) << 8
            | (value.b * 255.0) as i32
    }
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
