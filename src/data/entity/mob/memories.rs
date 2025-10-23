use crate::data::{PositionInDimension, entity::McUuid};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Memories {
    Allay {
        item_pickup_cooldown_ticks: Option<ValueHolder<i32>>,
        liked_noteblock: Option<ValueHolder<PositionInDimension>>,
        liked_noteblock_cooldown_ticks: Option<ValueHolder<i32>>,
        liked_player: Option<ValueHolder<McUuid>>,
    },
    Axolotl {
        has_hunting_cooldown: Option<EphemeralValueHolder<bool>>,
        play_dead_ticks: Option<ValueHolder<i32>>,
        #[serde(flatten)]
        temptation: Temptation,
    },
    Camel {
        gaze_cooldown_ticks: Option<ValueHolder<i32>>,
        #[serde(flatten)]
        temptation: Temptation,
    },
    CopperGolem {
        is_panicking: Option<ValueHolder<bool>>,
        visited_block_positions: Option<EphemeralValueHolder<Vec<PositionInDimension>>>,
        gaze_cooldown_ticks: Option<ValueHolder<i32>>,
    },
    Frog {
        is_in_water: Option<ValueHolder<EmptyCompound>>,
        is_pregnant: Option<ValueHolder<EmptyCompound>>,
    },
    Goat {
        long_jump_cooling_down: Option<ValueHolder<i32>>,
        ram_cooldown_ticcks: Option<ValueHolder<i32>>,
        #[serde(flatten)]
        temptation: Temptation,
    },
    Piglin {
        admiring_disabled: Option<EphemeralValueHolder<bool>>,
        admiring_item: Option<EphemeralValueHolder<bool>>,
        angry_at: Option<EphemeralValueHolder<McUuid>>,
        hunted_reccently: Option<EphemeralValueHolder<bool>>,
        universal_anger: Option<EphemeralValueHolder<bool>>,
    },
    PiglinBrute {
        angry_at: Option<EphemeralValueHolder<McUuid>>,
        home: Option<ValueHolder<PositionInDimension>>,
    },
    Sniffer {
        sniffer_explored_positions: Option<ValueHolder<Vec<PositionInDimension>>>,
        sniff_cooldown: Option<EphemeralValueHolder<EmptyCompound>>,
    },
    Villager {
        home: Option<ValueHolder<PositionInDimension>>,
        job_site: Option<ValueHolder<PositionInDimension>>,
        last_slept: Option<ValueHolder<i64>>,
        last_woken: Option<ValueHolder<i64>>,
        last_worked_at_poi: Option<ValueHolder<i64>>,
        meeting_point: Option<ValueHolder<PositionInDimension>>,
        // Boxed at recommendation of compiler
        potential_job_site: Option<Box<ValueHolder<PositionInDimension>>>,
        golem_detected_recently: Option<EphemeralValueHolder<bool>>,
    },
    Warden {
        is_emerging: Option<ValueHolder<EmptyCompound>>,
        dig_cooldown: Option<EphemeralValueHolder<EmptyCompound>>,
        is_sniffing: Option<ValueHolder<EmptyCompound>>,
        recent_projectile: Option<EphemeralValueHolder<EmptyCompound>>,
        roar_sound_cooldown: Option<EphemeralValueHolder<EmptyCompound>>,
        roar_sound_delay: Option<EphemeralValueHolder<EmptyCompound>>,
        touch_cooldown: Option<EphemeralValueHolder<EmptyCompound>>,
        vibration_cooldown: Option<EphemeralValueHolder<EmptyCompound>>,
        sniff_cooldown: Option<EphemeralValueHolder<EmptyCompound>>,
    },
}

#[derive(Clone)]
pub struct EmptyCompound;

impl Serialize for EmptyCompound {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        fastnbt::Value::Compound(HashMap::new()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EmptyCompound {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map = HashMap::<String, fastnbt::Value>::deserialize(deserializer)?;
        if map.is_empty() {
            Ok(Self)
        } else {
            Err(serde::de::Error::custom("Expected empty compound"))
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Temptation {
    is_tempted: Option<ValueHolder<bool>>,
    temptation_cooldown_ticks: Option<ValueHolder<i32>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ValueHolder<T> {
    value: T,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EphemeralValueHolder<T> {
    value: T,
    ttl: i64,
}
