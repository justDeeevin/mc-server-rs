use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::data::entity::mob::Color;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TropicalFish {
    from_bucket: bool,
    variant: Variant,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(try_from = "i32", into = "i32")]
pub struct Variant {
    pattern: Option<Pattern>,
    base_color: Color,
    pattern_color: Color,
}

#[derive(Clone, FromRepr)]
// TODO: test conversion Self <-> i16
pub enum Pattern {
    Flopper,
    Kob,
    Stripey,
    Sunstreak,
    Glitter,
    Snooper,
    Blockfish,
    Dasher,
    Betty,
    Brinely,
    Clayfish,
    Spotty,
}

impl From<Pattern> for i16 {
    fn from(value: Pattern) -> Self {
        use Pattern::*;
        let size = match value {
            Flopper | Stripey | Glitter | Blockfish | Betty | Clayfish => 1,
            _ => 0,
        };
        let variant = match value {
            Flopper | Kob => 0,
            Stripey | Sunstreak => 1,
            Glitter | Snooper => 2,
            Blockfish | Dasher => 3,
            Betty | Brinely => 4,
            Clayfish | Spotty => 5,
        };

        size | variant << 8
    }
}

impl Pattern {
    pub fn from_i16(value: i16) -> Result<Option<Self>, &'static str> {
        if value < 0 {
            Err("Invalid pattern")
        } else {
            Ok(Self::from_repr(value as usize))
        }
    }
}

impl TryFrom<i32> for Variant {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let pattern: Option<Pattern> = Pattern::from_i16(value as i16)?;
        let base_color = Color::try_from((value >> 16) as i8)?;
        let pattern_color = Color::try_from((value >> 24) as i8)?;

        Ok(Self {
            pattern,
            base_color,
            pattern_color,
        })
    }
}

impl From<Variant> for i32 {
    fn from(value: Variant) -> Self {
        let mut out = 0;
        if let Some(pattern) = value.pattern {
            out |= i16::from(pattern) as i32;
        }
        out |= (value.base_color as i32) << 16;
        out |= (value.pattern_color as i32) << 24;

        out
    }
}
