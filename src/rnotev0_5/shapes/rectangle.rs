use p2d::bounding_volume::AABB;
use serde::{Deserialize, Serialize};

use super::Line;
use crate::rnotev0_5::Transform;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "rectangle")]
/// A rectangle
pub struct Rectangle {
    #[serde(rename = "cuboid")]
    /// The cuboid, consisting of half extents.
    pub cuboid: p2d::shape::Cuboid,
    #[serde(rename = "transform")]
    /// The transform to place the rect in a coordinate space
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
    /// New from bounds
    pub fn from_p2d_aabb(bounds: AABB) -> Self {
        let cuboid = p2d::shape::Cuboid::new(bounds.half_extents());
        let transform = Transform::new_w_isometry(na::Isometry2::new(bounds.center().coords, 0.0));

        Self { cuboid, transform }
    }

    /// The outline lines of the rect
    pub fn outline_lines(&self) -> [Line; 4] {
        let upper_left = self.transform.transform_point(na::point![
            -self.cuboid.half_extents[0],
            -self.cuboid.half_extents[1]
        ]);
        let upper_right = self.transform.transform_point(na::point![
            self.cuboid.half_extents[0],
            -self.cuboid.half_extents[1]
        ]);
        let lower_left = self.transform.transform_point(na::point![
            -self.cuboid.half_extents[0],
            self.cuboid.half_extents[1]
        ]);
        let lower_right = self.transform.transform_point(na::point![
            self.cuboid.half_extents[0],
            self.cuboid.half_extents[1]
        ]);

        [
            Line {
                start: upper_left.coords,
                end: lower_left.coords,
            },
            Line {
                start: lower_left.coords,
                end: lower_right.coords,
            },
            Line {
                start: lower_right.coords,
                end: upper_right.coords,
            },
            Line {
                start: upper_right.coords,
                end: upper_left.coords,
            },
        ]
    }
}

impl From<crate::rnotev0_4::shapes::Rectangle> for Rectangle {
    fn from(rect: crate::rnotev0_4::shapes::Rectangle) -> Self {
        Self { cuboid: rect.cuboid, transform: rect.transform }
    }
}