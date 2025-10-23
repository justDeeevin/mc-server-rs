use crate::data::{
    Identifier,
    entity::mob::{Angerable, Breedable, Color, Tameable},
};
use serde::{Deserialize, Serialize};
use strum::{EnumString, IntoStaticStr};

#[derive(Serialize, Deserialize, Clone)]
pub struct Wolf {
    #[serde(flatten)]
    angerable: Angerable,
    #[serde(flatten)]
    tameable: Tameable,
    #[serde(flatten)]
    breedable: Breedable,
    #[serde(rename = "CollarColor", default = "default_color")]
    collar_color: Color,
    #[serde(default)]
    variant: Variant,
    // TODO: is this an identifier?
    sound_variant: SoundVariant,
}

const fn default_color() -> Color {
    Color::Red
}

#[derive(Serialize, Deserialize, Clone, Default, IntoStaticStr, EnumString)]
#[strum(serialize_all = "snake_case")]
#[serde(into = "Identifier", try_from = "Identifier")]
pub enum Variant {
    #[default]
    Pale,
    Ashen,
    Black,
    Chestnut,
    Rusty,
    Snowy,
    Spotted,
    Striped,
    Woods,
}

impl From<Variant> for Identifier {
    fn from(value: Variant) -> Self {
        <&'static str>::from(value).parse().unwrap()
    }
}

impl TryFrom<Identifier> for Variant {
    type Error = &'static str;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        let path = value.path;
        path.parse().map_err(|_| "Invalid variant")
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SoundVariant {
    Classic,
    Angry,
    Big,
    Cute,
    Grumpy,
    Puglin,
    Sad,
}
