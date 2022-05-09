use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use super::style::Style;
use crate::rnotev0_5::penpath::{Element, PenPath, Segment};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "brushstroke")]
pub struct BrushStroke {
    #[serde(rename = "path")]
    pub path: PenPath,
    #[serde(rename = "style")]
    pub style: Style,
}

impl Default for BrushStroke {
    fn default() -> Self {
        Self::new(
            Segment::Dot {
                element: Element::default(),
            },
            Style::default(),
        )
    }
}

impl BrushStroke {
    /// when one of the extents of the stroke is above this threshold, images are generated seperately for each stroke segment (to avoid very large images)
    pub const IMAGES_SEGMENTS_THRESHOLD: f64 = 1000.0;

    pub fn new(segment: Segment, style: Style) -> Self {
        let path = PenPath::new_w_segment(segment);

        Self::from_penpath(path, style).unwrap()
    }

    pub fn from_penpath(path: PenPath, style: Style) -> Option<Self> {
        if path.is_empty() {
            return None;
        }
        let new_brushstroke = Self { path, style };
        Some(new_brushstroke)
    }

    pub fn push_segment(&mut self, segment: Segment) {
        self.path.push_back(segment);
    }
}

impl From<crate::rnotev0_4::strokes::brushstroke::BrushStroke> for BrushStroke {
    fn from(bs: crate::rnotev0_4::strokes::brushstroke::BrushStroke) -> Self {
        let style = match bs.style {
            crate::rnotev0_4::strokes::brushstroke::BrushStrokeStyle::Marker { options } => {
                Style::Smooth(options.into())
            }
            crate::rnotev0_4::strokes::brushstroke::BrushStrokeStyle::Solid { options } => {
                Style::Smooth(options.into())
            }
            crate::rnotev0_4::strokes::brushstroke::BrushStrokeStyle::Textured { options } => {
                Style::Textured(options.into())
            }
        };

        let path = if bs.elements.len() == 1 {
            let elem = bs.elements.into_iter().next().unwrap();

            PenPath(
                std::iter::once(Segment::Dot {
                    element: elem.into(),
                })
                .collect(),
            )
        } else {
            let mut segs = VecDeque::with_capacity(bs.elements.len() / 2);

            let mut iter = bs.elements.into_iter();
            let first: Element = iter.next().unwrap().into();
            let mut last = Some(first);

            segs.push_back(Segment::Line {
                start: first,
                end: first,
            });

            while let Some(elem) = iter.next() {
                let elem: Element = elem.into();
                let last_elem = last.unwrap();

                // let cp1 = last_elem.pos.lerp(&elem.pos, 0.333);
                // let cp2 = last_elem.pos.lerp(&elem.pos, 0.666);

                segs.push_back(Segment::Line {
                    start: last_elem,
                    end: elem,
                });

                last = Some(elem);
            }

            PenPath(segs)
        };

        Self { path, style }
    }
}
