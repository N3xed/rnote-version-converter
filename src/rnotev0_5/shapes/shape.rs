use serde::{Deserialize, Serialize};

use super::{CubicBezier, Ellipse, Line, QuadraticBezier, Rectangle};
use crate::rnotev0_5::penpath::Segment;

// Container type to store shapes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "shape")]
/// A Shape type, holding the actual shape inside it
pub enum Shape {
    #[serde(rename = "line")]
    /// A line shape
    Line(Line),
    #[serde(rename = "rect")]
    /// A rectangle shape
    Rectangle(Rectangle),
    #[serde(rename = "ellipse")]
    /// An ellipse shape
    Ellipse(Ellipse),
    #[serde(rename = "quadbez")]
    /// A quadratic bezier curve shape
    QuadraticBezier(QuadraticBezier),
    #[serde(rename = "cubbez")]
    /// A cubic bezier curve shape
    CubicBezier(CubicBezier),
    #[serde(rename = "segment")]
    /// A segment
    Segment(Segment),
}

impl Default for Shape {
    fn default() -> Self {
        Self::Line(Line::default())
    }
}

impl From<crate::rnotev0_4::strokes::shapestroke::Shape> for Shape {
    fn from(shape: crate::rnotev0_4::strokes::shapestroke::Shape) -> Self {
        match shape {
            crate::rnotev0_4::strokes::shapestroke::Shape::Line(line) => Self::Line(line.into()),
            crate::rnotev0_4::strokes::shapestroke::Shape::Rectangle(rect) => Self::Rectangle(rect.into()),
            crate::rnotev0_4::strokes::shapestroke::Shape::Ellipse(ellipse) => Self::Ellipse(ellipse.into())
        }
    }
}