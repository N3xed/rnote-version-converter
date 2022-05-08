use serde::{Deserialize, Serialize};

use super::Element;

/// A single segment (usually of a path), containing elements to be able to being drawn with variable width
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "segment")]
pub enum Segment {
    #[serde(rename = "dot")]
    /// A dot segment.
    Dot {
        #[serde(rename = "element")]
        /// The element of the dot
        element: Element,
    },
    #[serde(rename = "line")]
    /// A line segment
    Line {
        #[serde(rename = "start")]
        /// The line start
        start: Element,
        #[serde(rename = "end")]
        /// The line end
        end: Element,
    },
    #[serde(rename = "quadbez")]
    /// A quadratic bezier segment
    QuadBez {
        #[serde(rename = "start")]
        /// The quadratic curve start
        start: Element,
        #[serde(rename = "cp")]
        /// The quadratic curve control point
        cp: na::Vector2<f64>,
        #[serde(rename = "end")]
        /// The quadratic curve end
        end: Element,
    },
    #[serde(rename = "cubbez")]
    /// A cubic bezier segment.
    CubBez {
        #[serde(rename = "start")]
        /// The cubic curve start
        start: Element,
        #[serde(rename = "cp1")]
        /// The cubic curve first control point
        cp1: na::Vector2<f64>,
        #[serde(rename = "cp2")]
        /// The cubic curve second control point
        cp2: na::Vector2<f64>,
        #[serde(rename = "end")]
        /// The cubic curve end
        end: Element,
    },
}

impl Segment {
    /// All segment choices have a start
    pub fn start(&self) -> Element {
        match self {
            Segment::Dot { element } => *element,
            Segment::Line { start, .. } => *start,
            Segment::QuadBez { start, .. } => *start,
            Segment::CubBez { start, .. } => *start,
        }
    }

    /// All segment choices have an end
    pub fn end(&self) -> Element {
        match self {
            Segment::Dot { element } => *element,
            Segment::Line { end, .. } => *end,
            Segment::QuadBez { end, .. } => *end,
            Segment::CubBez { end, .. } => *end,
        }
    }
}
