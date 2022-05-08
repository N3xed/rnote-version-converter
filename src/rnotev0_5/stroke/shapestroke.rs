use serde::{Deserialize, Serialize};

use super::style::Style;
use crate::rnotev0_5::shapes::Shape;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "shapestroke")]
pub struct ShapeStroke {
    #[serde(rename = "shape")]
    pub shape: Shape,
    #[serde(rename = "style")]
    pub style: Style,
}

impl ShapeStroke {
    pub fn new(shape: Shape, style: Style) -> Self {
        Self { shape, style }
    }
}

impl From<crate::rnotev0_4::strokes::shapestroke::ShapeStroke> for ShapeStroke {
    fn from(ss: crate::rnotev0_4::strokes::shapestroke::ShapeStroke) -> Self {
        let style = match ss.drawstyle {
            crate::rnotev0_4::strokes::shapestroke::ShapeDrawStyle::Rough { options } => {
                Style::Rough(options.into())
            }
            crate::rnotev0_4::strokes::shapestroke::ShapeDrawStyle::Smooth { options } => {
                Style::Smooth(options.into())
            }
        };

        Self {
            shape: ss.shape.into(),
            style,
        }
    }
}
