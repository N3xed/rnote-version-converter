use p2d::bounding_volume::AABB;
use serde::{Deserialize, Serialize};
use crate::rnotev0_4::geometry::AABBHelpers;

use super::element::Element;
use super::{SmoothOptions, TexturedOptions};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "brushstroke_style")]
pub enum BrushStrokeStyle {
    #[serde(rename = "marker")]
    Marker {
        #[serde(rename = "options")]
        options: SmoothOptions,
    },
    #[serde(rename = "solid")]
    Solid {
        #[serde(rename = "options")]
        options: SmoothOptions,
    },
    #[serde(rename = "textured")]
    Textured {
        #[serde(rename = "options")]
        options: TexturedOptions,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "brushstroke")]
pub struct BrushStroke {
    #[serde(rename = "elements")]
    pub elements: Vec<Element>,
    #[serde(rename = "style")]
    pub style: BrushStrokeStyle,
    #[serde(rename = "bounds")]
    pub bounds: AABB,
}

impl Default for BrushStroke {
    fn default() -> Self {
        BrushStroke {
            elements: vec![],
            style: BrushStrokeStyle::Solid {
                options: SmoothOptions::default(),
            },
            bounds: AABB::new_zero(),
        }
    }
}

impl BrushStroke {
    pub const HITBOX_DEFAULT: f64 = 10.0;
}