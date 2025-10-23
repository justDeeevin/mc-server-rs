use serde::{Deserialize, Serialize};
use strum::FromRepr;

#[derive(Serialize, Deserialize, Clone)]
pub struct CopperGolem {
    weather_state: WeatherState,
    next_weather_age: i64,
}

#[derive(Serialize, Deserialize, Clone, FromRepr)]
#[serde(into = "i32", try_from = "i32")]
pub enum WeatherState {
    Unaffected,
    Exposed,
    Weathered,
    Oxidized,
}

impl TryFrom<i32> for WeatherState {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_repr(value as usize).ok_or("Invalid weather state")
    }
}

impl From<WeatherState> for i32 {
    fn from(value: WeatherState) -> Self {
        value as i32
    }
}
