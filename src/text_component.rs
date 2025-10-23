use crate::{
    OneOf,
    data::{Coordinates, Identifier, Item, NbtPath, Selector},
};
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, FromInto, serde_as};
use std::{path::PathBuf, str::FromStr};
use strum::{EnumString, IntoStaticStr};

#[derive(Serialize, Deserialize, Deref, DerefMut, AsRef, AsMut, Clone)]
#[serde(try_from = "OneOf<TextComponentInner, OneOf<Vec<TextComponent>, String>>")]
#[repr(transparent)]
pub struct TextComponent(TextComponentInner);

impl TryFrom<OneOf<TextComponentInner, OneOf<Vec<TextComponent>, String>>> for TextComponent {
    type Error = &'static str;

    fn try_from(
        value: OneOf<TextComponentInner, OneOf<Vec<TextComponent>, String>>,
    ) -> Result<Self, Self::Error> {
        Ok(match value {
            OneOf::Left(inner) => Self(inner),
            OneOf::Right(OneOf::Left(vec)) => Self(vec.try_into()?),
            OneOf::Right(OneOf::Right(string)) => Self(string.into()),
        })
    }
}

impl<T: Into<TextComponentInner>> From<T> for TextComponent {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextComponentInner {
    // TODO: Test this!! sketchy af
    #[serde(flatten)]
    #[serde_as(as = "FromInto<OneOf<TextComponentContent, TextComponentContentUntagged>>")]
    content: TextComponentContent,
    #[serde(default)]
    extra: Vec<TextComponent>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    color: Option<TextComponentColor>,
    font: Option<PathBuf>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    shadow_color: Option<ARGB>,
    insertion: Option<String>,
    #[serde(rename = "clickEvent")]
    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "action", content = "contents", rename_all = "snake_case")]
pub enum HoverEvent {
    // Boxed for indirection
    ShowText(Box<TextComponent>),
    // Boxed at recommendation of compiler
    ShowItem(Box<Item>),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "OneOf<u32, [f32; 4]>", into = "[f32; 4]")]
pub struct ARGB {
    pub a: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<OneOf<u32, [f32; 4]>> for ARGB {
    fn from(value: OneOf<u32, [f32; 4]>) -> Self {
        match value {
            OneOf::Left(int) => Self {
                a: ((int >> 24) as u8) as f32 / 255.0,
                r: ((int >> 16) as u8) as f32 / 255.0,
                g: ((int >> 8) as u8) as f32 / 255.0,
                b: (int as u8) as f32 / 255.0,
            },
            OneOf::Right(floats) => Self {
                a: floats[0],
                r: floats[1],
                g: floats[2],
                b: floats[3],
            },
        }
    }
}

impl From<ARGB> for [f32; 4] {
    fn from(value: ARGB) -> Self {
        [value.a, value.r, value.g, value.b]
    }
}

impl From<String> for TextComponentInner {
    fn from(value: String) -> Self {
        Self {
            content: TextComponentContent::Text { text: value },
            extra: vec![],
            color: None,
            font: None,
            bold: Some(false),
            italic: Some(false),
            underlined: Some(false),
            strikethrough: Some(false),
            obfuscated: Some(false),
            shadow_color: None,
            insertion: None,
            click_event: None,
            hover_event: None,
        }
    }
}

impl From<&str> for TextComponentInner {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl TryFrom<Vec<TextComponent>> for TextComponentInner {
    type Error = &'static str;

    fn try_from(mut value: Vec<TextComponent>) -> Result<Self, Self::Error> {
        let mut out = value
            .pop()
            .ok_or("At least one component must be provided")?
            .0;
        out.extra = value;

        Ok(out)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ClickEvent {
    action: ClickEventAction,
    value: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ClickEventAction {
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    ChangePage,
    CopyToClipboard,
}

#[derive(EnumString, IntoStaticStr, Clone, Copy, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(try_from = "Identifier", into = "Identifier")]
pub enum DialogKind {
    Notice,
    Confirmation,
    MultiAction,
    ServerLinks,
    DialogList,
}

impl From<DialogKind> for Identifier {
    fn from(value: DialogKind) -> Self {
        Self {
            namespace: "minecraft".to_string(),
            path: <&'static str>::from(value).to_string(),
        }
    }
}
impl TryFrom<Identifier> for DialogKind {
    type Error = &'static str;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        if value.namespace != "minecraft" {
            return Err("Invalid namespace");
        }

        value.path.parse().map_err(|_| "Invalid dialog type")
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum TextComponentContent {
    Text { text: String },
    Translatable(TranslatableText),
    Score { score: Score },
    Selector(SelectorComponent),
    // TODO: keybind identifiers
    Keybind { keybind: String },
    Nbt(NbtComponent),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum TextComponentContentUntagged {
    Text { text: String },
    Translatable(TranslatableText),
    Score { score: Score },
    Selector(SelectorComponent),
    Keybind { keybind: String },
    Nbt(NbtComponent),
}

impl From<OneOf<TextComponentContent, TextComponentContentUntagged>> for TextComponentContent {
    fn from(value: OneOf<TextComponentContent, TextComponentContentUntagged>) -> Self {
        match value {
            OneOf::Left(content) => content,
            OneOf::Right(content) => content.into(),
        }
    }
}

impl From<TextComponentContentUntagged> for TextComponentContent {
    fn from(value: TextComponentContentUntagged) -> Self {
        match value {
            TextComponentContentUntagged::Text { text } => Self::Text { text },
            TextComponentContentUntagged::Translatable(translatable) => {
                Self::Translatable(translatable)
            }
            TextComponentContentUntagged::Score { score } => Self::Score { score },
            TextComponentContentUntagged::Selector(selector) => Self::Selector(selector),
            TextComponentContentUntagged::Keybind { keybind } => Self::Keybind { keybind },
            TextComponentContentUntagged::Nbt(nbt) => Self::Nbt(nbt),
        }
    }
}

impl From<TextComponentContent> for OneOf<TextComponentContent, TextComponentContentUntagged> {
    fn from(value: TextComponentContent) -> Self {
        Self::Left(value)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NbtComponent {
    #[serde(flatten)]
    nbt: Option<NbtData>,
    // Boxed for indirection
    // TODO: should this be gray?
    #[serde(default = "default_separator")]
    separator: Box<TextComponent>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NbtData {
    source: NbtSource,
    #[serde(flatten)]
    locator: Option<NbtLocator>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NbtLocator {
    nbt: NbtPath,
    interpret: Option<bool>,
    block: Coordinates,
    entity: Selector,
    storage: Identifier,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NbtSource {
    Block,
    Entity,
    Storage,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SelectorComponent {
    selector: Selector,
    #[serde(default = "default_separator")]
    // Boxed for indirection
    separator: Box<TextComponent>,
}

fn default_separator() -> Box<TextComponent> {
    Box::new(TextComponent(TextComponentInner {
        color: Some("gray".parse().unwrap()),
        ..", ".into()
    }))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TranslatableText {
    translate: Identifier,
    fallback: Option<String>,
    #[serde(default)]
    with: Vec<Option<TextComponent>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Score {
    name: String,
    objective: String,
}

#[derive(Clone)]
pub struct TextComponentColor(String);

impl std::fmt::Display for TextComponentColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for TextComponentColor {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.strip_prefix("#<")
            .and_then(|s| s.strip_suffix('>'))
            .is_some_and(|s| u32::from_str_radix(s, 16).is_ok())
            || matches!(
                s,
                "black"
                    | "dark_blue"
                    | "dark_green"
                    | "dark_aqua"
                    | "dark_red"
                    | "dark_purple"
                    | "gold"
                    | "gray"
                    | "dark_gray"
                    | "blue"
                    | "green"
                    | "aqua"
                    | "red"
                    | "light_purple"
                    | "yellow"
                    | "white"
            )
        {
            Ok(Self(s.to_string()))
        } else {
            Err("Invalid color")
        }
    }
}
