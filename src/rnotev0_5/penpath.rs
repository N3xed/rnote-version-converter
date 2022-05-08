pub mod element;
pub mod segment;

// Re exports
use std::collections::VecDeque;
use std::ops::{Deref, DerefMut};

pub use element::Element;
pub use segment::Segment;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "pen_path")]
/// a pen path, consisting of segments of pen input elements
pub struct PenPath(pub VecDeque<Segment>);

impl Deref for PenPath {
    type Target = VecDeque<Segment>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PenPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PenPath {
    /// A new pen path with a first dot segment
    pub fn new_w_dot(element: Element) -> Self {
        Self::new_w_segment(Segment::Dot { element })
    }

    /// A new pen path with a first segment
    pub fn new_w_segment(segment: Segment) -> Self {
        let mut segment_vec = VecDeque::with_capacity(1);
        segment_vec.push_back(segment);

        Self(segment_vec)
    }

    /// extracts the elements from the path. the path shape will be lost, as only the actual input elements are returned.
    pub fn into_elements(self) -> Vec<Element> {
        self.0
            .into_iter()
            .map(|segment| match segment {
                Segment::Dot { element: pos } => vec![pos],
                Segment::Line { start, end } => vec![start, end],
                Segment::QuadBez { start, cp: _, end } => vec![start, end],
                Segment::CubBez {
                    start,
                    cp1: _,
                    cp2: _,
                    end,
                } => vec![start, end],
            })
            .flatten()
            .collect()
    }
}

impl std::iter::FromIterator<Segment> for PenPath {
    fn from_iter<T: IntoIterator<Item = Segment>>(iter: T) -> Self {
        Self(VecDeque::from_iter(iter))
    }
}
