use serde::{Deserialize, Serialize};

use super::Rectangle;
use crate::rnotev0_5::Transform;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "line")]
/// A line
pub struct Line {
    #[serde(rename = "start")]
    /// The line start
    pub start: na::Vector2<f64>,
    #[serde(rename = "end")]
    /// The line end
    pub end: na::Vector2<f64>,
}

impl Line {
    /// creates a rect in the direction of the line, with a constant given width
    pub fn line_w_width_to_rect(&self, width: f64) -> Rectangle {
        let vec = self.end - self.start;
        let magn = vec.magnitude();
        let angle = na::Rotation2::rotation_between(&na::Vector2::x(), &vec).angle();

        Rectangle {
            cuboid: p2d::shape::Cuboid::new(na::vector![magn * 0.5, width * 0.5]),
            transform: Transform::new_w_isometry(na::Isometry2::new(self.start + vec * 0.5, angle)),
        }
    }

    /// Splits itself given the no splits
    pub fn split(&self, n_splits: i32) -> Vec<Self> {
        (0..n_splits)
            .map(|i| {
                let sub_start = self
                    .start
                    .lerp(&self.end, f64::from(i) / f64::from(n_splits));
                let sub_end = self
                    .start
                    .lerp(&self.end, f64::from(i + 1) / f64::from(n_splits));

                Line {
                    start: sub_start,
                    end: sub_end,
                }
            })
            .collect::<Vec<Self>>()
    }
}

impl From<crate::rnotev0_4::curves::Line> for Line {
    fn from(line: crate::rnotev0_4::curves::Line) -> Self {
        Self {
            start: line.start,
            end: line.end,
        }
    }
}
