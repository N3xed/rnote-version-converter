use p2d::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

use crate::rnotev0_4::{shapes, geometry::AABBHelpers};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename = "bitmapimage_format")]
pub enum BitmapImageFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
}

impl BitmapImageFormat {
    pub fn as_mime_type(&self) -> String {
        match self {
            BitmapImageFormat::Png => String::from("image/png"),
            BitmapImageFormat::Jpeg => String::from("image/jpeg"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "bitmapimage")]
pub struct BitmapImage {
    #[serde(rename = "data_base64")]
    pub data_base64: String,
    #[serde(rename = "format")]
    pub format: BitmapImageFormat,
    #[serde(rename = "intrinsic_size")]
    pub intrinsic_size: [f64; 2],
    #[serde(rename = "rectangle")]
    pub rectangle: shapes::Rectangle,
    #[serde(rename = "bounds")]
    pub bounds: AABB,
}

impl Default for BitmapImage {
    fn default() -> Self {
        Self {
            data_base64: String::default(),
            format: BitmapImageFormat::Png,
            intrinsic_size: [0.0, 0.0],
            rectangle: shapes::Rectangle::default(),
            bounds: AABB::new_zero(),
        }
    }
}

impl BitmapImage {
    pub const OFFSET_X_DEFAULT: f64 = 32.0;
    pub const OFFSET_Y_DEFAULT: f64 = 32.0;
}
