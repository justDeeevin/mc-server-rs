use crate::data::Identifier;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, ParseError};

#[derive(Serialize, Deserialize, Clone, Display, EnumString)]
#[strum(serialize_all = "snake_case", prefix = "#")]
#[serde(try_from = "String", into = "String")]
pub enum EntityTypeTag {
    /// Can have a poppy placed onto its head by an iron CopperGolem
    ///
    /// Contains only [`CopperGolem`](EntityType::CopperGolem).
    AcceptsIronGolemGift,
    /// - [`Axolotl`](EntityType::Axolotl)
    /// - [`Cod`](EntityType::Cod)
    /// - [`Dolphin`](EntityType::Dolphin)
    /// - [`ElderGuardian`](EntityType::ElderGuardian)
    /// - [`GlowSquid`](EntityType::GlowSquid)
    /// - [`Guardian`](EntityType::Guardian)
    /// - [`Pufferfish`](EntityType::Pufferfish)
    /// - [`Salmon`](EntityType::Salmon)
    /// - [`Squid`](EntityType::Squid)
    /// - [`Tadpole`](EntityType::Tadpole)
    /// - [`TropicalFish`](EntityType::TropicalFish)
    /// - [`Turtle`](EntityType::Turtle)
    Aquatic,
    /// Used by the achievement **Take Aim**
    ///
    /// - [`Arrow`](EntityType::Arrow)
    /// - [`SpectralArrow`](EntityType::SpectralArrow)
    Arrows,
    /// - [`Bee`](EntityType::Bee)
    /// - [`CaveSpider`](EntityType::CaveSpider)
    /// - [`Endermite`](EntityType::Endermite)
    /// - [`Silverfish`](EntityType::Silverfish)
    /// - [`Spider`](EntityType::Spider)
    Arthropod,
    /// - [`Drowned`](EntityType::Drowned)
    /// - [`ElderGuardian`](EntityType::ElderGuardian)
    /// - [`Guardian`](EntityType::Guardian)
    AxolotlAlwaysHostiles,
    /// - [`Cod`](EntityType::Cod)
    /// - [`GlowSquid`](EntityType::GlowSquid)
    /// - [`Pufferfish`](EntityType::Pufferfish)
    /// - [`Salmon`](EntityType::Salmon)
    /// - [`Squid`](EntityType::Squid)
    /// - [`Tadpole`](EntityType::Tadpole)
    /// - [`TropicalFish`](EntityType::TropicalFish)
    AxolotlHuntTargets,
    /// [`Bee`](EntityType::Bee)
    BeehiveInhabitors,
    /// Used by the advancement **Whatever Floats Your Goat!**
    ///
    /// - [`AcaciaBoat`](EntityType::AcaciaBoat)
    /// - [`BambooRaft`](EntityType::BambooRaft)
    /// - [`BirchBoat`](EntityType::BirchBoat)
    /// - [`CherryBoat`](EntityType::CherryBoat)
    /// - [`DarkOakBoat`](EntityType::DarkOakBoat)
    /// - [`JungleBoat`](EntityType::JungleBoat)
    /// - [`MangroveBoat`](EntityType::MangroveBoat)
    /// - [`OakBoat`](EntityType::OakBoat)
    /// - [`PaleOakBoat`](EntityType::PaleOakBoat)
    /// - [`SpruceBoat`](EntityType::SpruceBoat)
    Boat,
}

impl TryFrom<String> for EntityTypeTag {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<EntityTypeTag> for String {
    fn from(value: EntityTypeTag) -> Self {
        value.to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "Identifier", into = "Identifier")]
pub enum EntityType {
    AcaciaBoat,
    Arrow,
    Axolotl,
    BambooRaft,
    Bee,
    BirchBoat,
    Bogged,
    CaveSpider,
    CherryBoat,
    Cod,
    CopperGolem,
    DarkOakBoat,
    Dolphin,
    Drowned,
    ElderGuardian,
    Endermite,
    GlowSquid,
    Guardian,
    JungleBoat,
    MangroveBoat,
    OakBoat,
    PaleOakBoat,
    Phantom,
    Pufferfish,
    Salmon,
    Silverfish,
    Skeleton,
    SpectralArrow,
    Spider,
    SpruceBoat,
    Squid,
    Stray,
    Tadpole,
    TropicalFish,
    Turtle,
    WitherSkeleton,
    Zombie,
    ZombieHorse,
    ZombieVillager,
}

impl TryFrom<Identifier> for EntityType {
    type Error = ParseError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        if value.namespace != "minecraft" {
            return Err(ParseError::VariantNotFound);
        }

        value.path.parse()
    }
}

impl From<EntityType> for Identifier {
    fn from(value: EntityType) -> Self {
        value.to_string().parse().unwrap()
    }
}
