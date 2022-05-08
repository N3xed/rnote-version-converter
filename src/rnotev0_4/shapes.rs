use parry2d_f64::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

use super::transform::Transform;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "rectangle")]
pub struct Rectangle {
    #[serde(rename = "cuboid")]
    pub cuboid: p2d::shape::Cuboid,
    #[serde(rename = "transform")]
    pub transform: Transform,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            cuboid: p2d::shape::Cuboid::new(na::Vector2::zeros()),
            transform: Transform::default(),
        }
    }
}

impl Rectangle {
    pub fn global_aabb(&self) -> AABB {
        let center = self.transform.transform * na::point![0.0, 0.0];
        // using a vector to ignore the translation
        let half_extents = na::Vector2::from_homogeneous(
            self.transform.transform.into_inner().abs() * self.cuboid.half_extents.to_homogeneous(),
        )
        .unwrap();

        AABB::from_half_extents(center, half_extents)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "ellipse")]
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

impl Ellipse {
    pub fn global_aabb(&self) -> AABB {
        let center = self.transform.transform * na::point![0.0, 0.0];
        // using a vector to ignore the translation
        let half_extents = na::Vector2::from_homogeneous(
            self.transform.transform.into_inner().abs() * self.radii.to_homogeneous(),
        )
        .unwrap();

        AABB::from_half_extents(center, half_extents)
    }
}
