use p2d::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

use crate::rnotev0_4::{shapes, geometry::AABBHelpers};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "vectorimage")]
pub struct VectorImage {
    #[serde(rename = "svg_data")]
    pub svg_data: String,
    #[serde(rename = "intrinsic_size")]
    pub intrinsic_size: na::Vector2<f64>,
    #[serde(rename = "rectangle")]
    pub rectangle: shapes::Rectangle,
    #[serde(rename = "bounds")]
    pub bounds: AABB,
}

impl Default for VectorImage {
    fn default() -> Self {
        Self {
            svg_data: String::default(),
            intrinsic_size: na::Vector2::zeros(),
            rectangle: shapes::Rectangle::default(),
            bounds: AABB::new_zero(),
        }
    }
}

impl VectorImage {
    pub const SIZE_X_DEFAULT: f64 = 500.0;
    pub const SIZE_Y_DEFAULT: f64 = 500.0;
    pub const OFFSET_X_DEFAULT: f64 = 32.0;
    pub const OFFSET_Y_DEFAULT: f64 = 32.0;
}
