use serde::{Deserialize, Serialize};

use crate::rnotev0_5::Transform;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "ellipse")]
/// A Ellipse
pub struct Ellipse {
    /// The radii of the ellipse
    #[serde(rename = "radii")]
    pub radii: na::Vector2<f64>,
    /// The transform
    #[serde(rename = "transform")]
    pub transform: Transform,
}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            radii: na::Vector2::zeros(),
            transform: Transform::default(),
        }
    }
}

impl From<crate::rnotev0_4::shapes::Ellipse> for Ellipse {
    fn from(ellipse: crate::rnotev0_4::shapes::Ellipse) -> Self {
        Self {
            radii: ellipse.radii,
            transform: ellipse.transform.into(),
        }
    }
}
