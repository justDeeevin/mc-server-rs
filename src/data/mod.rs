pub mod block;
pub mod components;
pub mod entity;
pub mod particle;
pub mod player;

use components::MinecraftComponents;
use derive_more::{AsMut, AsRef, Deref, DerefMut};
use num_traits::{PrimInt, Signed};
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;
use std::{collections::HashMap, str::FromStr};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    id: Identifier,
    count: Option<i32>,
    components: Option<Components>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Components {
    #[serde(flatten)]
    minecraft: MinecraftNamespaceWrapper<MinecraftComponents>,
    #[serde(flatten)]
    other: HashMap<Identifier, fastnbt::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ItemWithSlot<I: PrimInt + Signed = i8> {
    slot: I,
    #[serde(flatten)]
    item: Item,
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
#[serde(try_from = "String", into = "String")]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

impl From<Identifier> for String {
    fn from(value: Identifier) -> Self {
        format!("{}:{}", value.namespace, value.path)
    }
}

impl TryFrom<String> for Identifier {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl FromStr for Identifier {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<_>>();
        if parts.len() == 2 {
            Ok(Self {
                namespace: parts[0].to_string(),
                path: parts[1].to_string(),
            })
        } else if parts.len() == 1 {
            Ok(Self {
                namespace: "minecraft".to_string(),
                path: parts[0].to_string(),
            })
        } else {
            Err("Invalid identifier")
        }
    }
}

// TODO: arguments
#[derive(Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub enum Selector {
    NearestPlayer,
    RandomPlayer,
    AllPlayers,
    AllEntities,
    Executor,
    NearestEntity,
}

impl std::fmt::Display for Selector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NearestPlayer => write!(f, "@p"),
            Self::RandomPlayer => write!(f, "@r"),
            Self::AllPlayers => write!(f, "@a"),
            Self::AllEntities => write!(f, "@e"),
            Self::Executor => write!(f, "@s"),
            Self::NearestEntity => write!(f, "@n"),
        }
    }
}

impl From<Selector> for String {
    fn from(value: Selector) -> Self {
        value.to_string()
    }
}

impl FromStr for Selector {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "@p" => Ok(Self::NearestPlayer),
            "@r" => Ok(Self::RandomPlayer),
            "@a" => Ok(Self::AllPlayers),
            "@e" => Ok(Self::AllEntities),
            "@s" => Ok(Self::Executor),
            "@n" => Ok(Self::NearestEntity),
            _ => Err("Invalid selector"),
        }
    }
}

impl TryFrom<String> for Selector {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

// TODO
#[derive(Serialize, Deserialize, Clone)]
pub struct NbtPath;

#[derive(Serialize, Deserialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub struct Coordinates {
    x: Coordinate,
    y: Coordinate,
    z: Coordinate,
}

impl std::fmt::Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl From<Coordinates> for String {
    fn from(value: Coordinates) -> Self {
        value.to_string()
    }
}

impl FromStr for Coordinates {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err("Invalid coordinates");
        }

        Ok(Self {
            x: parts.pop().unwrap().parse()?,
            y: parts.pop().unwrap().parse()?,
            z: parts.pop().unwrap().parse()?,
        })
    }
}

impl TryFrom<String> for Coordinates {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[derive(Clone)]
pub struct Coordinate {
    value: f64,
    kind: CoordinateKind,
}

impl std::fmt::Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.kind.prefix(), self.value)
    }
}

impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s.chars().next() {
            Some('~') => CoordinateKind::Relative,
            Some('^') => CoordinateKind::Local,
            Some(_) => CoordinateKind::Absolute,
            None => return Err("Invalid coordinate"),
        };

        let value = if kind == CoordinateKind::Absolute {
            s.parse::<f64>()
        } else {
            s[1..].parse::<f64>()
        }
        .map_err(|_| "Invalid coordinate")?;

        Ok(Self { value, kind })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CoordinateKind {
    Absolute,
    Relative,
    Local,
}

impl CoordinateKind {
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Absolute => "",
            Self::Relative => "~",
            Self::Local => "^",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Deref, DerefMut, AsRef, AsMut)]
pub struct MinecraftNamespaceWrapper<T: Serialize + for<'a> Deserialize<'a>>(
    #[serde(with = "prefix_minecraft_namespace")] pub T,
);

with_prefix!(prefix_minecraft_namespace "minecraft:");

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionInDimension {
    dimension: Identifier,
    pos: [i32; 3],
}
