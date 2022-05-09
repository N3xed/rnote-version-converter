use serde::{Deserialize, Serialize};

use self::bitmapimage::BitmapImage;
use self::brushstroke::BrushStroke;
use self::shapestroke::ShapeStroke;
use self::vectorimage::VectorImage;

mod bitmapimage;
mod brushstroke;
mod shapestroke;
mod style;
mod vectorimage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "stroke")]
pub enum Stroke {
    #[serde(rename = "brushstroke")]
    BrushStroke(BrushStroke),
    #[serde(rename = "shapestroke")]
    ShapeStroke(ShapeStroke),
    #[serde(rename = "vectorimage")]
    VectorImage(VectorImage),
    #[serde(rename = "bitmapimage")]
    BitmapImage(BitmapImage),
}

impl Default for Stroke {
    fn default() -> Self {
        Self::BrushStroke(BrushStroke::default())
    }
}

impl TryFrom<crate::rnotev0_4::StrokeStyle> for Stroke {
    type Error = anyhow::Error;
    fn try_from(s: crate::rnotev0_4::StrokeStyle) -> anyhow::Result<Stroke> {
        use crate::rnotev0_4::StrokeStyle as SSv4;
        match s {
            SSv4::BrushStroke(s) => Ok(Stroke::BrushStroke(s.into())),
            SSv4::ShapeStroke(s) => Ok(Stroke::ShapeStroke(s.into())),
            SSv4::VectorImage(s) => Ok(Stroke::VectorImage(s.into())),
            SSv4::BitmapImage(s) => Ok(Stroke::BitmapImage(TryInto::<BitmapImage>::try_into(s)?)),
        }
    }
}
