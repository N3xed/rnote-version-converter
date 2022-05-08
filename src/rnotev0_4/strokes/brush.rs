use serde::{Deserialize, Serialize};

use super::{SmoothOptions, TexturedOptions};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
#[serde(rename = "brushstyle")]
pub enum BrushStyle {
    #[serde(rename = "marker")]
    Marker,
    #[serde(rename = "solid")]
    Solid,
    #[serde(rename = "textured")]
    Textured,
}

impl Default for BrushStyle {
    fn default() -> Self {
        Self::Solid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "brush")]
pub struct Brush {
    #[serde(rename = "style")]
    pub style: BrushStyle,
    #[serde(rename = "smooth_options")]
    pub smooth_options: SmoothOptions,
    #[serde(rename = "textured_options")]
    pub textured_options: TexturedOptions,
}

impl Default for Brush {
    fn default() -> Self {
        Self {
            style: BrushStyle::default(),
            smooth_options: SmoothOptions::default(),
            textured_options: TexturedOptions::default(),
        }
    }
}
