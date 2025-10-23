pub mod allay;
pub mod armadillo;
pub mod armor_stand;
pub mod axolotl;
pub mod cat;
pub mod copper_golem;
pub mod ender_dragon;
pub mod enderman;
pub mod frog;
pub mod horse;
pub mod llama;
pub mod mannequin;
pub mod memories;
pub mod mooshroom;
pub mod panda;
pub mod parrot;
pub mod pufferfish;
pub mod rabbit;
pub mod raider;
pub mod salmon;
pub mod shulker;
pub mod tropical_fish;
pub mod villager;
pub mod warden;
pub mod wolf;
pub mod zombie;

use crate::{
    OneOf,
    data::{
        Identifier, Item, MinecraftNamespaceWrapper,
        components::AttributeModifierOperation,
        entity::{
            McUuid,
            mob::{
                allay::Allay,
                armadillo::Armadillo,
                armor_stand::ArmorStand,
                copper_golem::CopperGolem,
                ender_dragon::EnderDragon,
                enderman::Enderman,
                horse::{ChestedHorse, Horse},
                llama::Llama,
                mannequin::Mannequin,
                memories::Memories,
                mooshroom::Mooshroom,
                panda::Panda,
                parrot::Parrot,
                pufferfish::Pufferfish,
                rabbit::Rabbit,
                raider::Raider,
                salmon::Salmon,
                shulker::Shulker,
                tropical_fish::TropicalFish,
                villager::{Offers, Villager},
                warden::Warden,
                wolf::Wolf,
                zombie::Zombie,
            },
        },
        player::Player,
    },
};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Mob {
    absorption_amount: f32,
    #[serde(rename = "active_effects", default)]
    active_effects: Vec<PotionEffect>,
    #[serde(rename = "attributes", default)]
    attributes: Vec<MobAttribute>,
    brain: Brain,
    can_pick_up_loot: bool,
    death_time: i16,
    #[serde(default)]
    armor_items: ArmorItems,
    #[serde(default)]
    hand_items: HandItems,
    #[serde(rename = "body_armor_item")]
    body_armor_item: Option<Item>,
    fall_flying: I8Bool,
    health: f32,
    hurt_by_timestamp: i32,
    hurt_time: i16,
    #[serde(rename = "leash")]
    leash: Option<OneOf<[i32; 3], UuidHolder>>,
    left_handed: bool,
    #[serde(rename = "NoAI")]
    no_ai: bool,
    persistence_required: bool,
    sleeping_x: i32,
    sleeping_y: i32,
    sleeping_z: i32,
    // FIXME: only used when spawning a mob!! :<
    team: Option<String>,
    #[serde(flatten)]
    unique: Option<OneOf<LivingMob, DummyMob>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum DummyMob {
    ArmorStand(ArmorStand),
    // Boxed at recommendation of compiler
    Mannequin(Box<Mannequin>),
    // Funny to describe a player as a "dummy", but it is missing the same fields
    // Boxed at recommendation of compiler
    Player(Box<Player>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LivingMob {
    #[serde(rename = "CanPickUpLoot")]
    can_pick_up_loot: bool,
    #[serde(rename = "LeftHanded")]
    left_handed: bool,
    persistence_required: bool,
    leash: Option<OneOf<[i32; 3], UuidHolder>>,
    armor_drop_chances: ArmorDropChances,
    hand_drop_chances: HandDropChances,
    #[serde(rename = "body_armor_drop_chance")]
    body_armor_drop_chance: i8,
    #[serde(rename = "NoAI")]
    no_ai: bool,
    #[serde(flatten)]
    unique: Option<UniqueMob>,
}
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(into = "[Option<Item>; 4]", from = "[Option<Item>; 4]")]
pub struct ArmorItems {
    feet: Option<Item>,
    legs: Option<Item>,
    chest: Option<Item>,
    head: Option<Item>,
}

impl From<[Option<Item>; 4]> for ArmorItems {
    fn from(value: [Option<Item>; 4]) -> Self {
        let [feet, legs, chest, head] = value;
        Self {
            feet,
            legs,
            chest,
            head,
        }
    }
}

impl From<ArmorItems> for [Option<Item>; 4] {
    fn from(value: ArmorItems) -> Self {
        [value.feet, value.legs, value.chest, value.head]
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(into = "[Option<Item>; 2]", from = "[Option<Item>; 2]")]
pub struct HandItems {
    mainhand: Option<Item>,
    offhand: Option<Item>,
}

impl From<[Option<Item>; 2]> for HandItems {
    fn from(value: [Option<Item>; 2]) -> Self {
        let [mainhand, offhand] = value;
        Self { mainhand, offhand }
    }
}

impl From<HandItems> for [Option<Item>; 2] {
    fn from(value: HandItems) -> Self {
        [value.mainhand, value.offhand]
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(into = "[f32; 4]", from = "[f32; 4]")]
pub struct ArmorDropChances {
    feet: f32,
    legs: f32,
    chest: f32,
    head: f32,
}

impl From<[f32; 4]> for ArmorDropChances {
    fn from(value: [f32; 4]) -> Self {
        Self {
            feet: value[0],
            legs: value[1],
            chest: value[2],
            head: value[3],
        }
    }
}

impl From<ArmorDropChances> for [f32; 4] {
    fn from(value: ArmorDropChances) -> Self {
        [value.feet, value.legs, value.chest, value.head]
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(into = "[f32; 2]", from = "[f32; 2]")]
pub struct HandDropChances {
    mainhand: f32,
    offhand: f32,
}

impl From<[f32; 2]> for HandDropChances {
    fn from(value: [f32; 2]) -> Self {
        Self {
            mainhand: value[0],
            offhand: value[1],
        }
    }
}

impl From<HandDropChances> for [f32; 2] {
    fn from(value: HandDropChances) -> Self {
        [value.mainhand, value.offhand]
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UniqueMob {
    Allay(Allay),
    Armadillo(Armadillo),
    ArmorStand(ArmorStand),
    #[serde(rename_all = "PascalCase")]
    Axolotl {
        from_bucket: I8Bool,
        variant: i32,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Bat {
        bat_flags: I8Bool,
    },
    #[serde(rename_all = "PascalCase")]
    Bee {
        cannot_enter_hive_ticks: i32,
        crops_grown_since_pollination: i32,
        #[serde(rename = "flower_pos")]
        flower_pos: [i32; 3],
        has_nectar: bool,
        #[serde(rename = "hive_pos")]
        hive_pos: [i32; 3],
        ticks_since_pollination: i32,
        #[serde(flatten)]
        breedable: Breedable,
        #[serde(flatten)]
        angerable: Angerable,
    },
    #[serde(rename_all = "PascalCase")]
    Camel {
        #[serde(flatten)]
        horse: Horse,
        last_pose_tick: i64,
    },
    #[serde(rename_all = "PascalCase")]
    Cat {
        collar_color: i8,
        #[serde(rename = "variant")]
        variant: Identifier,
        #[serde(flatten)]
        breedable: Breedable,
        #[serde(flatten)]
        tameable: Tameable,
    },
    #[serde(rename_all = "PascalCase")]
    Chicken {
        egg_lay_time: i32,
        is_chicken_jockey: bool,
        #[serde(rename = "variant")]
        variant: Identifier,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Cod {
        from_bucket: I8Bool,
    },
    CopperGolem(CopperGolem),
    Cow {
        variant: Identifier,
        #[serde(flatten)]
        breedable: Breedable,
    },
    Creeper {
        #[serde(rename = "ExplosionRadius")]
        explosion_radius: i8,
        #[serde(rename = "Fuse")]
        fuse: i16,
        ignited: I8Bool,
        powered: I8Bool,
    },
    #[serde(rename_all = "PascalCase")]
    Dolphin {
        moistness: i32,
        got_fish: I8Bool,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Donkey {
        #[serde(flatten)]
        horse: Horse,
        #[serde(flatten)]
        chested_horse: ChestedHorse,
    },
    Drowned(Zombie),
    EnderDragon(EnderDragon),
    Enderman(Enderman),
    #[serde(rename_all = "PascalCase")]
    Endermite {
        lifetime: i32,
    },
    #[serde(rename_all = "PascalCase")]
    Evoker {
        spell_ticks: i32,
        #[serde(flatten)]
        raider: Raider,
    },
    #[serde(rename_all = "PascalCase")]
    Fox {
        crouching: I8Bool,
        sitting: I8Bool,
        sleeping: I8Bool,
        #[serde(default)]
        trusted: Vec<McUuid>,
        #[serde(rename = "Type")]
        kind: Identifier,
        #[serde(flatten)]
        breedable: Breedable,
    },
    Frog {
        variant: Identifier,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Ghast {
        explosion_power: i8,
    },
    #[serde(rename_all = "PascalCase")]
    GlowSquid {
        dark_ticks_remaining: i32,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Goat {
        has_left_horn: I8Bool,
        has_right_horn: I8Bool,
        is_screaming_goat: I8Bool,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Hoglin {
        cannot_be_hunted: bool,
        is_immune_to_zombification: bool,
        time_in_overworld: i32,
    },
    #[serde(rename_all = "PascalCase")]
    Horse {
        #[serde(flatten)]
        horse: Horse,
        variant: i32,
    },
    Husk(Zombie),
    #[serde(rename_all = "PascalCase")]
    Illusioner {
        spell_ticks: i32,
        #[serde(flatten)]
        raider: Raider,
    },
    #[serde(rename_all = "PascalCase")]
    IronGolem {
        player_created: I8Bool,
        #[serde(flatten)]
        angerable: Angerable,
    },
    Llama(Llama),
    #[serde(rename_all = "PascalCase")]
    MagmaCube {
        size: i32,
        #[serde(rename = "wasOnGround")]
        was_on_ground: I8Bool,
    },
    Mooshroom(Mooshroom),
    Mule {
        #[serde(flatten)]
        horse: Horse,
        #[serde(flatten)]
        chested_horse: ChestedHorse,
    },
    #[serde(rename_all = "PascalCase")]
    Ocelot {
        trusting: bool,
        #[serde(flatten)]
        breedable: Breedable,
    },
    Panda(Panda),
    Parrot(Parrot),
    Phantom {
        size: i32,
        anchor_pos: Option<[i32; 3]>,
    },
    Pig {
        // TODO: this is probably an identifier?
        variant: String,
        #[serde(flatten)]
        breedable: Breedable,
    },
    #[serde(rename_all = "PascalCase")]
    Piglin {
        cannot_hunt: bool,
        #[serde(default)]
        inventory: Vec<Item>,
        #[serde(default)]
        is_baby: bool,
        is_immune_to_zombification: bool,
        time_in_overworld: i32,
    },
    #[serde(rename_all = "PascalCase")]
    PiglinBrute {
        is_immune_to_zombification: bool,
        time_in_overworld: i32,
    },
    #[serde(rename_all = "PascalCase")]
    Pillager {
        #[serde(default)]
        inventory: Vec<Item>,
        #[serde(flatten)]
        raider: Raider,
    },
    PolarBear {
        #[serde(flatten)]
        angerable: Angerable,
        #[serde(flatten)]
        breedable: Breedable,
    },
    Pufferfish(Pufferfish),
    Rabbit(Rabbit),
    #[serde(rename_all = "PascalCase")]
    Ravager {
        #[serde(flatten)]
        raider: Raider,
        attack_tick: i32,
        roar_tick: i32,
        stun_tick: i32,
    },
    Salmon(Salmon),
    #[serde(rename_all = "PascalCase")]
    Sheep {
        #[serde(default = "default_sheep_color")]
        color: Color,
        sheared: bool,
    },
    Shulker(Shulker),
    #[serde(rename_all = "PascalCase")]
    Skeleton {
        stray_conversion_time: i32,
    },
    #[serde(rename_all = "PascalCase")]
    SkeletonHorse {
        #[serde(flatten)]
        horse: Horse,
        skeleton_trap: bool,
        skeleton_trap_time: i32,
    },
    #[serde(rename_all = "camelCase")]
    Slime {
        #[serde(rename = "Size")]
        size: i32,
        was_on_ground: bool,
    },
    Sniffer(Breedable),
    #[serde(rename_all = "PascalCase")]
    SnowGolem {
        pumpkin: bool,
    },
    Strider(Breedable),
    Squid(Breedable),
    #[serde(rename_all = "PascalCase")]
    Tadpole {
        age: i32,
        from_bucket: bool,
    },
    TraderLlama(Llama),
    TropicalFish(TropicalFish),
    Turtle {
        #[serde(flatten)]
        breedable: Breedable,
        has_egg: bool,
    },
    Vex {
        bound_pos: Option<[i32; 3]>,
        life_ticks: i32,
    },
    Villager(Villager),
    #[serde(rename_all = "PascalCase")]
    Vindicator {
        #[serde(flatten)]
        raider: Raider,
        johnny: bool,
    },
    #[serde(rename_all = "PascalCase")]
    WanderingTrader {
        despawn_delay: i32,
        offers: Offers,
        #[serde(rename = "wander_target")]
        wander_target: [i32; 3],
        inventory: Vec<Item>,
    },
    Warden(Warden),
    Witch(Raider),
    #[serde(rename_all = "PascalCase")]
    Wither {
        invul: i32,
    },
    Wolf(Wolf),
    #[serde(rename_all = "PascalCase")]
    Zoglin {
        is_baby: bool,
    },
    Zombie(Zombie),
    ZombieHorse(Horse),
    #[serde(rename_all = "PascalCase")]
    ZombieVillager {
        #[serde(flatten)]
        villager: Villager,
        #[serde(flatten)]
        zombie: Zombie,
        conversion_time: i32,
        conversion_player: McUuid,
    },
    ZombifiedPiglin {
        #[serde(flatten)]
        angerable: Angerable,
        #[serde(flatten)]
        zombie: Zombie,
    },
}

const fn default_sheep_color() -> Color {
    Color::White
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Tameable {
    owner: Option<McUuid>,
    sitting: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Breedable {
    age: i32,
    forced_age: i32,
    in_love: i32,
    love_cause: McUuid,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Angerable {
    anger_time: i32,
    angry_at: McUuid,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct UuidHolder {
    uuid: McUuid,
}

#[derive(Clone, Copy, Deref, DerefMut, AsRef, AsMut)]
pub struct I8Bool(bool);

impl Serialize for I8Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i8(if **self { 1 } else { 0 })
    }
}

impl<'de> Deserialize<'de> for I8Bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let i8 = i8::deserialize(deserializer)?;
        Ok(Self(i8 != 0))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Brain {
    memories: Option<MinecraftNamespaceWrapper<Memories>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MobAttribute {
    id: String,
    base: f64,
    #[serde(default)]
    modifiers: Vec<MobAttributeModifier>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MobAttributeModifier {
    amount: Option<f64>,
    id: Option<Identifier>,
    operation: Option<AttributeModifierOperation>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PotionEffect {
    ambient: Option<bool>,
    amplifier: Option<i8>,
    duration: Option<i32>,
    hidden_effect: Option<Box<Self>>,
    id: Option<Identifier>,
    show_icon: Option<bool>,
    show_particles: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(into = "i8", try_from = "i8")]
pub enum Color {
    White,
    Orange,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    LightGray,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
}

impl TryFrom<i8> for Color {
    type Error = &'static str;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid collar color")
    }
}

impl From<Color> for i8 {
    fn from(value: Color) -> Self {
        value as i8
    }
}
