use super::Identifier;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumString)]
#[strum(serialize_all = "snake_case", prefix = "#")]
#[serde(into = "String", try_from = "String")]
pub enum DamageTypeTag {
    /// [`#IsExplosion`](DamageTypeTag::IsExplosion)
    AlwaysHurtsEnderDragons,
    /// - [`Arrow`](DamageType::Arrow)
    /// - [`Fireball`](DamageType::Fireball)
    /// - [`Trident`](DamageType::Trident)
    /// - [`WindCharge`](DamageType::WindCharge)
    /// - [`WitherSkull`](DamageType::WitherSkull)
    AlwaysKillsArmorStands,
    /// Maximum priority for figuring out which fall should be used for death message.
    ///
    /// Only contains [`OutOfWorld`](DamageType::OutOfWorld).
    AlwaysMostSignificantFall,
    /// [`Magic`](DamageType::Magic)
    AlwaysTriggersSilverfish,
    /// - [`#IsExplosion`](DamageTypeTag::IsExplosion)
    /// - [`Magic`](DamageType::Magic)
    /// - [`Thorns`](DamageType::Thorns)
    AvoidsGuardianThorns,
    /// [`OnFire`](DamageType::OnFire)
    BurnsArmorStands,
    /// - [`Campfire`](DamageType::Campfire)
    /// - [`HotFloor`](DamageType::HotFloor)
    BurnFromStepping,
    /// - [`Cramming`](DamageType::Cramming)
    /// - [`DragonBreath`](DamageType::DragonBreath)
    /// - [`Drown`](DamageType::Drown)
    /// - [`EnderPearl`](DamageType::EnderPearl)
    /// - [`Fall`](DamageType::Fall)
    /// - [`FlyIntoWall`](DamageType::FlyIntoWall)
    /// - [`Freeze`](DamageType::Freeze)
    /// - [`Generic`](DamageType::Generic)
    /// - [`GenericKill`](DamageType::GenericKill)
    /// - [`InWall`](DamageType::InWall)
    /// - [`IndirectMagic`](DamageType::IndirectMagic)
    /// - [`Magic`](DamageType::Magic)
    /// - [`OnFire`](DamageType::OnFire)
    /// - [`OutOfWorld`](DamageType::OutOfWorld)
    /// - [`OutsideBorder`](DamageType::OutsideBorder)
    /// - [`SonicBoom`](DamageType::SonicBoom)
    /// - [`Stalagmite`](DamageType::Stalagmite)
    /// - [`Starve`](DamageType::Starve)
    /// - [`Wither`](DamageType::Wither)
    BypassesArmor,
    /// Bypasses I-frames
    BypassesCooldown,
    /// Bypasses resistance potion and enchantments
    ///
    /// Only contains [`Starve`](DamageType::Starve).
    BypassesEffects,
    /// Bypasses only enchantments
    ///
    /// Only contains [`SonicBoom`](DamageType::SonicBoom).
    BypassesEnchantments,
    /// - [`GenericKill`](DamageType::GenericKill)
    /// - [`OutOfWorld`](DamageType::OutOfWorld)
    BypassesInvulnerability,
    /// Bypasses only resistance potion
    ///
    /// - [`GenericKill`](DamageType::GenericKill)
    /// - [`OutOfWorld`](DamageType::OutOfWorld)
    BypassesResistance,
    /// - [`#BypassesArmor`](DamageTypeTag::BypassesArmor)
    /// - [`FallingAnvil`](DamageType::FallingAnvil)
    /// - [`FallingStalactite`](DamageType::FallingStalactite)
    BypassesShield,
    /// - [`#BypassesInvulnerability`](DamageTypeTag::BypassesInvulnerability)
    /// - [`Cramming`](DamageType::Cramming)
    /// - [`Drown`](DamageType::Drown)
    /// - [`DryOut`](DamageType::DryOut)
    /// - [`Freeze`](DamageType::Freeze)
    /// - [`InWall`](DamageType::InWall)
    /// - [`IndirectMagic`](DamageType::IndirectMagic)
    /// - [`Magic`](DamageType::Magic)
    /// - [`OutsideBorder`](DamageType::OutsideBorder)
    /// - [`Starve`](DamageType::Starve)
    /// - [`Thorns`](DamageType::Thorns)
    /// - [`Wither`](DamageType::Wither)
    BypassesWolfAmrmor,
    /// - [`#IsPlayerAttack`](DamageTypeTag::IsPlayerAttack)
    /// - [`PlayerExplosion`](DamageType::PlayerExplosion)
    CanBreakArmorStand,
    /// Damages helmets for 75% of the damage done.
    ///
    /// - [`FallingAnvil`](DamageType::FallingAnvil)
    /// - [`FallingBlock`](DamageType::FallingBlock)
    /// - [`FallingStalactite`](DamageType::FallingStalactite)
    DamagesHelmet,
    /// - [`Campfire`](DamageType::Campfire)
    /// - [`InFire`](DamageType::InFire)
    IgnitesArmorStands,
    /// Ignored if the player has water breathing or if the `drowningDamage` gamerule is set to
    /// false.
    ///
    /// Only contains [`Drown`](DamageType::Drown).
    IsDrowning,
    /// Reduced by blast protection
    ///
    /// - [`BadRespawnPoint`](DamageType::BadRespawnPoint)
    /// - [`Explosion`](DamageType::Explosion)
    /// - [`Fireworks`](DamageType::Fireworks)
    /// - [`PlayerExplosion`](DamageType::PlayerExplosion)
    IsExplosion,
    /// Ignored if the player has slow falling or if the `fallDamage` gamerule is set to false.
    /// Reduced by feather falling.
    ///
    /// - [`EnderPearl`](DamageType::EnderPearl)
    /// - [`Fall`](DamageType::Fall)
    /// - [`Stalagmite`](DamageType::Stalagmite)
    IsFall,
    /// Ignored if the player has fire resistance or if the `fireDamage` gamerule is set to false.
    /// Reduced by fire protection.
    ///
    /// - [`Campfire`](DamageType::Campfire)
    /// - [`Fireball`](DamageType::Fireball)
    /// - [`HotFloor`](DamageType::HotFloor)
    /// - [`InFire`](DamageType::InFire)
    /// - [`Lava`](DamageType::Lava)
    /// - [`OnFire`](DamageType::OnFire)
    /// - [`UnattributedFireball`](DamageType::UnattributedFireball)
    IsFire,
    /// Ignored if the player is wearing any piece of leather armor or if the `freezeDamage` gamerule
    /// is set to false.
    ///
    /// - [`Freeze`](DamageType::Freeze)
    IsFreezing,
    /// Used to make turtles drop bowls when killed by lightning.
    ///
    /// Only contains [`LightningBolt`](DamageType::LightningBolt).
    IsLightning,
    /// - [`MaceSmash`](DamageType::MaceSmash)
    /// - [`PlayerAttack`](DamageType::PlayerAttack)
    IsPlayerAttack,
    /// - Used in many advancements
    /// - Used to determine if an enderman should teleport
    /// - Used to check if a shulker was hit by a projectile preceeding the check for duplication
    /// - Reduced by projectile protection
    ///
    /// - [`Arrow`](DamageType::Arrow)
    /// - [`Fireball`](DamageType::Fireball)
    /// - [`MobProjectile`](DamageType::MobProjectile)
    /// - [`Thrown`](DamageType::Thrown)
    /// - [`Trident`](DamageType::Trident)
    /// - [`UnattributedFireball`](DamageType::UnattributedFireball)
    /// - [`WindCharge`](DamageType::WindCharge)
    /// - [`WitherSkull`](DamageType::WitherSkull)
    IsProjectile,
    /// [`MaceSmash`](DamageType::MaceSmash)
    MaceSmash,
    /// [`MobAttackNoAggro`](DamageType::MobAttackNoAggro)
    NoAnger,
    // TODO: how is this different from NoKnockback?
    /// Prevents entities from being hurt marked, preventing the server from syncing the velocity to the client.
    ///
    /// Contains only [`Drown`](DamageType::Drown).
    NoImpact,
    /// - [`BadRespawnPoint`](DamageType::BadRespawnPoint)
    /// - [`Cactus`](DamageType::Cactus)
    /// - [`Campfire`](DamageType::Campfire)
    /// - [`Cramming`](DamageType::Cramming)
    /// - [`DragonBreath`](DamageType::DragonBreath)
    /// - [`Drown`](DamageType::Drown)
    /// - [`DryOut`](DamageType::DryOut)
    /// - [`EnderPearl`](DamageType::EnderPearl)
    /// - [`Explosion`](DamageType::Explosion)
    /// - [`Fall`](DamageType::Fall)
    /// - [`FlyIntoWall`](DamageType::FlyIntoWall)
    /// - [`Freeze`](DamageType::Freeze)
    /// - [`Generic`](DamageType::Generic)
    /// - [`GenericKill`](DamageType::GenericKill)
    /// - [`HotFloor`](DamageType::HotFloor)
    /// - [`InFire`](DamageType::InFire)
    /// - [`InWall`](DamageType::InWall)
    /// - [`Lava`](DamageType::Lava)
    /// - [`LightningBolt`](DamageType::LightningBolt)
    /// - [`Magic`](DamageType::Magic)
    /// - [`OnFire`](DamageType::OnFire)
    /// - [`OutOfWorld`](DamageType::OutOfWorld)
    /// - [`OutsideBorder`](DamageType::OutsideBorder)
    /// - [`PlayerExplosion`](DamageType::PlayerExplosion)
    /// - [`Stalagmite`](DamageType::Stalagmite)
    /// - [`Starve`](DamageType::Starve)
    /// - [`SweetBerryBush`](DamageType::SweetBerryBush)
    /// - [`Wither`](DamageType::Wither)
    NoKnockback,
    /// - [`#IsPlayerAttack`](DamageTypeTag::IsPlayerAttack)
    /// - [`#PanicEnvironmentalCauses`](DamageTypeTag::PanicEnvironmentalCauses)
    /// - [`Arrow`](DamageType::Arrow)
    /// - [`DragonBreath`](DamageType::DragonBreath)
    /// - [`Explosion`](DamageType::Explosion)
    /// - [`Fireball`](DamageType::Fireball)
    /// - [`Fireworks`](DamageType::Fireworks)
    /// - [`IndirectMagic`](DamageType::IndirectMagic)
    /// - [`Magic`](DamageType::Magic)
    /// - [`MobAttack`](DamageType::MobAttack)
    /// - [`MobProjectile`](DamageType::MobProjectile)
    /// - [`PlayerExplosion`](DamageType::PlayerExplosion)
    /// - [`SonicBoom`](DamageType::SonicBoom)
    /// - [`Sting`](DamageType::Sting)
    /// - [`Thrown`](DamageType::Thrown)
    /// - [`Trident`](DamageType::Trident)
    /// - [`UnattributedFireball`](DamageType::UnattributedFireball)
    /// - [`WindCharge`](DamageType::WindCharge)
    /// - [`Wither`](DamageType::Wither)
    /// - [`WitherSkull`](DamageType::WitherSkull)
    PanicCauses,
    /// - [`Cactus`](DamageType::Cactus)
    /// - [`Freeze`](DamageType::Freeze)
    /// - [`HotFloor`](DamageType::HotFloor)
    /// - [`InFire`](DamageType::InFire)
    /// - [`Lava`](DamageType::Lava)
    /// - [`LightningBolt`](DamageType::LightningBolt)
    /// - [`OnFire`](DamageType::OnFire)
    PanicEnvironmentalCauses,
    /// - [`IndirectMagic`](DamageType::IndirectMagic)
    /// - [`Magic`](DamageType::Magic)
    /// - [`SonicBoom`](DamageType::SonicBoom)
    /// - [`Thorns`](DamageType::Thorns)
    WitchResistantTo,
    /// [`Drown`](DamageType::Drown)
    WitherImmuneTo,
}

impl DamageTypeTag {
    pub const fn contains(self, kind: DamageType) -> bool {
        use DamageType::*;
        match self {
            Self::AlwaysHurtsEnderDragons => Self::IsExplosion.contains(kind),
            Self::AlwaysKillsArmorStands => {
                matches!(kind, Arrow | Fireball | Trident | WindCharge | WitherSkull)
            }
            Self::AlwaysMostSignificantFall => matches!(kind, OutOfWorld),
            Self::AlwaysTriggersSilverfish => matches!(kind, Magic),
            Self::AvoidsGuardianThorns => {
                Self::IsExplosion.contains(kind) || matches!(kind, Magic | Thorns)
            }
            Self::BurnsArmorStands => matches!(kind, OnFire),
            Self::BurnFromStepping => matches!(kind, Campfire | HotFloor),
            Self::BypassesArmor => matches!(
                kind,
                Cramming
                    | DragonBreath
                    | Drown
                    | EnderPearl
                    | Fall
                    | FlyIntoWall
                    | Freeze
                    | Magic
                    | OnFire
                    | OutOfWorld
                    | OutsideBorder
                    | SonicBoom
                    | Stalagmite
                    | Starve
                    | Wither
            ),
            Self::BypassesCooldown => false,
            Self::BypassesEffects => matches!(kind, Starve),
            Self::BypassesEnchantments => matches!(kind, SonicBoom),
            Self::BypassesInvulnerability => matches!(kind, GenericKill | OutOfWorld),
            Self::BypassesResistance => matches!(kind, GenericKill | OutOfWorld),
            Self::BypassesShield => {
                Self::BypassesArmor.contains(kind)
                    || matches!(
                        kind,
                        Cactus
                            | Campfire
                            | DryOut
                            | FallingAnvil
                            | FallingStalactite
                            | HotFloor
                            | InFire
                            | Lava
                            | LightningBolt
                            | SweetBerryBush
                    )
            }
            Self::BypassesWolfAmrmor => {
                Self::BypassesArmor.contains(kind)
                    || matches!(
                        kind,
                        Cactus
                            | Campfire
                            | DryOut
                            | FallingAnvil
                            | FallingStalactite
                            | HotFloor
                            | InFire
                            | Lava
                            | LightningBolt
                            | SweetBerryBush
                    )
            }
            Self::CanBreakArmorStand => {
                Self::IsPlayerAttack.contains(kind) | matches!(kind, PlayerExplosion)
            }
            Self::DamagesHelmet => matches!(kind, FallingAnvil | FallingBlock | FallingStalactite),
            Self::IgnitesArmorStands => matches!(kind, Campfire | InFire),
            Self::IsDrowning => matches!(kind, Drown),
            Self::IsExplosion => matches!(
                kind,
                BadRespawnPoint | Explosion | Fireworks | PlayerExplosion
            ),
            Self::IsFall => matches!(kind, EnderPearl | Fall | Stalagmite),
            Self::IsFire => matches!(
                kind,
                Campfire | Fireball | HotFloor | InFire | OnFire | UnattributedFireball
            ),
            Self::IsFreezing => matches!(kind, Freeze),
            Self::IsLightning => matches!(kind, LightningBolt),
            Self::IsPlayerAttack => matches!(kind, MaceSmash | PlayerAttack),
            Self::IsProjectile => matches!(
                kind,
                Arrow
                    | Fireball
                    | MobProjectile
                    | Thrown
                    | Trident
                    | UnattributedFireball
                    | WindCharge
                    | WitherSkull
            ),
            Self::MaceSmash => matches!(kind, MaceSmash),
            Self::NoAnger => matches!(kind, MobAttackNoAggro),
            Self::NoImpact => matches!(kind, Drown),
            Self::NoKnockback => matches!(
                kind,
                BadRespawnPoint
                    | Cactus
                    | Campfire
                    | Cramming
                    | DragonBreath
                    | Drown
                    | DryOut
                    | EnderPearl
                    | Explosion
                    | Fall
                    | FlyIntoWall
                    | Freeze
                    | Generic
                    | GenericKill
                    | HotFloor
                    | InFire
                    | InWall
                    | Lava
                    | LightningBolt
                    | Magic
                    | OnFire
                    | OutOfWorld
                    | OutsideBorder
                    | PlayerExplosion
                    | Stalagmite
                    | Starve
                    | SweetBerryBush
                    | Wither
            ),
            Self::PanicCauses => {
                Self::IsPlayerAttack.contains(kind)
                    || Self::PanicEnvironmentalCauses.contains(kind)
                    || matches!(
                        kind,
                        Arrow
                            | DragonBreath
                            | Explosion
                            | Fireball
                            | Fireworks
                            | IndirectMagic
                            | Magic
                            | MobAttack
                            | MobProjectile
                            | PlayerExplosion
                            | SonicBoom
                            | Sting
                            | Thrown
                            | Trident
                            | UnattributedFireball
                            | WindCharge
                            | Wither
                            | WitherSkull
                    )
            }
            Self::PanicEnvironmentalCauses => matches!(
                kind,
                Cactus | Freeze | HotFloor | InFire | Lava | LightningBolt | OnFire
            ),
            Self::WitchResistantTo => matches!(kind, IndirectMagic | Magic | SonicBoom | Thorns),
            Self::WitherImmuneTo => matches!(kind, Drown),
        }
    }
}

impl TryFrom<String> for DamageTypeTag {
    type Error = strum::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl From<DamageTypeTag> for String {
    fn from(value: DamageTypeTag) -> Self {
        value.to_string()
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Display, EnumString, Default, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "Identifier", into = "Identifier")]
pub enum DamageType {
    /// - Arrow
    /// - Tipped arrow
    /// - Spectral arrow
    Arrow,
    /// Bed or respawn anchor explosion
    BadRespawnPoint,
    Cactus,
    /// Standing on a campfire or soul campfire
    Campfire,
    Cramming,
    /// Unused according to wiki
    DragonBreath,
    /// - Drowning
    /// - Water/rain for mobs that take damage from it
    ///   - Snow golems
    ///   - Blazes
    ///   - Endermen
    ///   - Striders
    /// - Sea creatures suffocating, specifically:
    ///   - Fish
    ///   - Tadpoles
    ///   - Dolphins
    ///   - Squid
    Drown,
    // FIXME: wiki conflicts on which damage type dolphin suffocation falls under
    /// Some newer sea creature suffocating:
    /// - Dolphins  
    /// - Axolotls
    DryOut,
    EnderPearl,
    /// - End crystal explosion
    /// - Damage dealt to the ender dragon when a healing end crystal is destroyed
    /// - Wither fight start
    /// - Minecart with TNT
    /// - Creeper
    /// - Ghast fireball
    /// - Wither skull
    /// - Dragon respawn
    Explosion,
    /// Fall damage
    Fall,
    FallingAnvil,
    /// Any non-anvil, non-dripstone falling block landing on a mob
    FallingBlock,
    /// Falling pointed dripstone
    FallingStalactite,
    /// A ghast or blaze fireball **with an owner** hitting a mob directly
    Fireball,
    Fireworks,
    FlyIntoWall,
    /// Powder snow tick damage
    Freeze,
    // TODO: what are player_hurt_entity and DamageEvent?
    /// - Bees dying after stinging
    /// - Simulating the `player_hurt_entity trigger when punching an interaction entity`
    /// - Used in a `DamageEvent` package
    /// - Damage dealt to the ender dragon when a healing end crystal is destroyed **with the
    ///   `kill` command**
    /// - Otherwise unassigned damage type
    #[default]
    Generic,
    /// `kill` command
    GenericKill,
    /// Magma block
    HotFloor,
    // TODO: wiki is conflicting on whether campfires are InFire or Campfire
    /// - Fire or soul fire
    /// - Standing on a campfire or soul campfire
    InFire,
    /// Suffocating in a solid block
    InWall,
    /// - Instant damage from a harming/healing potion
    /// - Extra damage from a guardian's beam
    /// - Evoker fangs **with an owner**
    /// - Splash water bottles hitting vulnerable mobs
    ///   - Snow golems
    ///   - Blazes
    ///   - Endermen
    ///   - Striders
    IndirectMagic,
    Lava,
    LightningBolt,
    MaceSmash,
    /// - `/loot kill`
    /// - Ticking damage from a harming/healing effect (**not initial contact**)
    /// - Ticking damage from a poison effect
    /// - Evoker fangs **with no owner**
    /// - Wither skull **with no owner** hitting someone
    /// - Conduit attack
    /// - Arrow of harming
    Magic,
    /// - Melee attack dealt by most mobs
    /// - Ender dragon pushing mobs away
    /// - Guardian beam
    /// - Ravager roar
    MobAttack,
    /// Goat ram
    MobAttackNoAggro,
    /// Shulker bullet
    MobProjectile,
    /// - Burning
    /// - Snow golems melting
    OnFire,
    /// Void tick damage
    OutOfWorld,
    OutsideBorder,
    // TODO: what is kill_mob_near_sculk_catalyst?
    /// - Player hitting an entity
    /// - Feeding a parrot a cookie
    /// - When granting the `kill_mob_near_sculk_catalyst` criteria if the actual damage type couldn't be found
    PlayerAttack,
    // TODO: how is this determined?
    /// Explosion for which a player was directly responsible
    PlayerExplosion,
    /// Warden sonic boom
    SonicBoom,
    /// Llama spit
    Spit,
    /// Falling onto pointed dripstone
    Stalagmite,
    /// - A vex summoned by an evoker dying after timeout
    /// - Starving
    Starve,
    /// A bee sting
    Sting,
    SweetBerryBush,
    Thorns,
    /// Being hit by
    /// - Snowball
    /// - Egg
    /// - Ender pearl
    Thrown,
    Trident,
    /// A ghast or blaze fireball **without an owner** hitting a mob directly
    UnattributedFireball,
    WindCharge,
    /// Wither effect
    Wither,
    /// A wither skull **with an owner** hitting a mob directly
    WitherSkull,
}

impl TryFrom<Identifier> for DamageType {
    type Error = strum::ParseError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        if value.namespace != "minecraft" {
            return Err(strum::ParseError::VariantNotFound);
        }
        value.path.parse()
    }
}

impl From<DamageType> for Identifier {
    fn from(value: DamageType) -> Self {
        value.to_string().parse().unwrap()
    }
}
