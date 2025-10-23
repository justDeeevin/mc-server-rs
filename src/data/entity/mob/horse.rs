use crate::data::{
    ItemWithSlot,
    entity::{McUuid, mob::Breedable},
};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Horse {
    bred: bool,
    eating_haystack: bool,
    owner: Option<McUuid>,
    tame: bool,
    temper: i32,
    #[serde(flatten)]
    breedable: Breedable,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChestedHorse {
    chested_horse: bool,
    #[serde(default)]
    items: Vec<ItemWithSlot>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(into = "i32", try_from = "i32")]
pub struct HorseColors {
    color: Color,
    markings: Markings,
}

#[derive(EnumIter, Clone, Copy)]
pub enum Color {
    White,
    Creamy,
    Chestnut,
    Brown,
    Black,
    Gray,
    DarkBrown,
}

impl TryFrom<i32> for Color {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Color::iter()
            .find(|color| *color as i32 == value)
            .ok_or("Invalid color")
    }
}

#[derive(EnumIter, Clone, Copy)]
pub enum Markings {
    None,
    White,
    WhiteField,
    WhiteDots,
    BlackDots,
}

impl TryFrom<i32> for Markings {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Markings::iter()
            .find(|markings| *markings as i32 == value)
            .ok_or("Invalid markings")
    }
}

impl From<HorseColors> for i32 {
    fn from(value: HorseColors) -> Self {
        (value.color as i32) | ((value.markings as i32) << 8)
    }
}

impl TryFrom<i32> for HorseColors {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let color = Color::try_from(value & 0xFF)?;
        let markings = Markings::try_from((value >> 8) & 0xFF)?;

        Ok(Self { color, markings })
    }
}
