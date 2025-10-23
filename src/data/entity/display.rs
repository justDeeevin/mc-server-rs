use crate::{
    OneOf,
    data::{Item, block::Block, entity::one_f32},
    text_component::{ARGB, TextComponent},
};
use glam::Mat4;
use serde::{Deserialize, Serialize};
use serde_with::{FromInto, serde_as};

#[serde_as]
#[derive(Serialize, Deserialize, Clone)]
pub struct Display {
    #[serde(default)]
    billboard: Pivot,
    brightness: Option<Brightness>,
    #[serde(default = "default_glow_color_override")]
    glow_color_override: i32,
    #[serde(default)]
    height: f32,
    #[serde(default)]
    width: f32,
    interpolation_duration: i32,
    teleport_duration: i32,
    // FIXME: not saved to entity, but can be set by commands
    start_interpolation: i32,
    #[serde(default)]
    shadow_radius: f32,
    #[serde(default = "one_f32")]
    shadow_strength: f32,
    #[serde(default = "one_f32")]
    view_range: f32,
    #[serde_as(as = "FromInto<OneOf<[f32; 16], Transformation>>")]
    transformation: Transformation,
    #[serde(flatten)]
    unique: Option<UniqueDisplay>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum UniqueDisplay {
    BlockDisplay {
        block_state: Block,
    },
    ItemDisplay {
        // Boxed at recommendation of compiler
        item: Box<Item>,
        #[serde(default)]
        item_display: ItemDisplay,
    },
    TextDisplay {
        #[serde(default)]
        alignment: TextAlignment,
        background: ARGB,
        #[serde(default)]
        default_background: bool,
        #[serde(default = "default_line_width")]
        line_width: i32,
        #[serde(default)]
        see_through: bool,
        #[serde(default)]
        shadow: bool,
        // TODO: wiki says this is stored as raw JSON text. I doubt it though...
        // Boxed at recommendation of compiler
        text: Box<TextComponent>,
        // TODO: test how this gets (de)serialized... the wiki is confusing about how unsigned is
        // handled in NBT
        text_opacity: u8,
    },
}

const fn default_line_width() -> i32 {
    200
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum TextAlignment {
    #[default]
    Center,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum ItemDisplay {
    #[default]
    None,
    ThirdpersonLefthand,
    ThirdpersonRighthand,
    FirstpersonLefthand,
    FirstpersonRighthand,
    Head,
    Gui,
    Ground,
    Fixed,
}

const fn default_glow_color_override() -> i32 {
    -1
}

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Pivot {
    #[default]
    Fixed,
    Vertical,
    Horizontal,
    Center,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Brightness {
    block: i32,
    sky: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transformation {
    right_rotation: Rotation,
    scale: [f32; 3],
    left_rotation: Rotation,
    translation: [f32; 3],
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(from = "OneOf<[f32; 4], AngleAxisRotation>")]
pub struct Rotation([f32; 4]);

#[derive(Serialize, Deserialize, Clone)]
pub struct AngleAxisRotation {
    angle: f32,
    axis: [f32; 3],
}

impl From<AngleAxisRotation> for Rotation {
    fn from(value: AngleAxisRotation) -> Self {
        let scalar = (value.angle / 2.0).cos();
        let sin = (value.angle / 2.0).sin();
        let x = value.axis[0] * sin;
        let y = value.axis[1] * sin;
        let z = value.axis[2] * sin;

        Self([x, y, z, scalar])
    }
}

impl From<OneOf<[f32; 4], AngleAxisRotation>> for Rotation {
    fn from(value: OneOf<[f32; 4], AngleAxisRotation>) -> Self {
        match value {
            OneOf::Left(value) => Self(value),
            OneOf::Right(value) => value.into(),
        }
    }
}

// TODO: is there shear? figure out how to test
impl From<[f32; 16]> for Transformation {
    fn from(value: [f32; 16]) -> Self {
        let mut matrix = Mat4::from_cols_array(&value);
        let w = matrix.w_axis.w;
        if w != 0.0 && w != 1.0 {
            matrix /= w;
        }

        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();

        Self {
            right_rotation: Rotation(rotation.to_array()),
            scale: scale.to_array(),
            left_rotation: Rotation([0.0, 0.0, 0.0, 1.0]),
            translation: translation.to_array(),
        }
    }
}

impl From<OneOf<[f32; 16], Transformation>> for Transformation {
    fn from(value: OneOf<[f32; 16], Transformation>) -> Self {
        match value {
            OneOf::Left(value) => value.into(),
            OneOf::Right(value) => value,
        }
    }
}

impl From<Transformation> for OneOf<[f32; 16], Transformation> {
    fn from(value: Transformation) -> Self {
        Self::Right(value)
    }
}
