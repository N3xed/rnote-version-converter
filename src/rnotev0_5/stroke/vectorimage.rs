use serde::{Deserialize, Serialize};

use crate::rnotev0_5::shapes::Rectangle;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "vectorimage")]
pub struct VectorImage {
    #[serde(rename = "svg_data")]
    pub svg_data: String,
    #[serde(rename = "intrinsic_size")]
    pub intrinsic_size: na::Vector2<f64>,
    #[serde(rename = "rectangle")]
    pub rectangle: Rectangle,
}

impl Default for VectorImage {
    fn default() -> Self {
        Self {
            svg_data: String::default(),
            intrinsic_size: na::Vector2::zeros(),
            rectangle: Rectangle::default(),
        }
    }
}

impl VectorImage {
    /// The default offset in surface coords when importing a vector image
    pub const IMPORT_OFFSET_DEFAULT: na::Vector2<f64> = na::vector![32.0, 32.0];
}

impl From<crate::rnotev0_4::strokes::vectorimage::VectorImage> for VectorImage {
    fn from(vi: crate::rnotev0_4::strokes::vectorimage::VectorImage) -> Self {
        Self {
            svg_data: vi.svg_data,
            intrinsic_size: vi.intrinsic_size,
            rectangle: vi.rectangle.into(),
        }
    }
}
