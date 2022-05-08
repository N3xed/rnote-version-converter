use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct InputData {
    pub pos: na::Vector2<f64>,
    pub pressure: f64,
}

impl Default for InputData {
    fn default() -> Self {
        Self {
            pos: na::vector![0.0, 0.0],
            pressure: Self::PRESSURE_DEFAULT,
        }
    }
}

impl InputData {
    pub const PRESSURE_DEFAULT: f64 = 0.5;
}
