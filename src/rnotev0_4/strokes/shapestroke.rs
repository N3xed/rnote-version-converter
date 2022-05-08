use parry2d_f64::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

use super::SmoothOptions;
use crate::rnotev0_4::geometry::AABBHelpers;
use crate::rnotev0_4::roughoptions::RoughOptions;
use crate::rnotev0_4::shapes::Rectangle;
use crate::rnotev0_4::{curves, shapes};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "shape")]
pub enum Shape {
    #[serde(rename = "line")]
    Line(curves::Line),
    #[serde(rename = "rectangle")]
    Rectangle(shapes::Rectangle),
    #[serde(rename = "ellipse")]
    Ellipse(shapes::Ellipse),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "shape_drawstyle")]
pub enum ShapeDrawStyle {
    #[serde(rename = "smooth")]
    Smooth {
        #[serde(rename = "options")]
        options: SmoothOptions,
    },
    #[serde(rename = "rough")]
    Rough {
        #[serde(rename = "options")]
        options: RoughOptions,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "shapestroke")]
pub struct ShapeStroke {
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    #[serde(rename = "shape")]
    pub shape: Shape,
    #[serde(rename = "drawstyle")]
    pub drawstyle: ShapeDrawStyle,
    #[serde(rename = "bounds")]
    pub bounds: AABB,
}

impl Default for ShapeStroke {
    fn default() -> Self {
        ShapeStroke {
            seed: None,
            shape: Shape::Rectangle(Rectangle::default()),
            drawstyle: ShapeDrawStyle::Smooth {
                options: SmoothOptions::default(),
            },
            bounds: AABB::new_zero(),
        }
    }
}
