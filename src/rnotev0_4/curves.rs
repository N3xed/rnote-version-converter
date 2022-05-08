use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "line")]
pub struct Line {
    #[serde(rename = "start")]
    pub start: na::Vector2<f64>,
    #[serde(rename = "end")]
    pub end: na::Vector2<f64>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct QuadBezier {
    pub start: na::Vector2<f64>,
    pub cp: na::Vector2<f64>,
    pub end: na::Vector2<f64>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct CubicBezier {
    pub start: na::Vector2<f64>,
    pub cp1: na::Vector2<f64>,
    pub cp2: na::Vector2<f64>,
    pub end: na::Vector2<f64>,
}
