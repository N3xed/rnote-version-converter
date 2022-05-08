use serde::{Deserialize, Serialize};

use super::textureddotsdistribution::TexturedDotsDistribution;
use crate::rnotev0_5::Color;

/// The Options of how a textured shape should look

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "textured_options")]
pub struct TexturedOptions {
    /// An optional seed to generate reproducable strokes
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    /// The width
    #[serde(rename = "stroke_width")]
    pub stroke_width: f64,
    /// The color of the stroke
    #[serde(rename = "stroke_color")]
    pub stroke_color: Option<Color>,
    /// Amount dots per 10x10 area
    #[serde(rename = "density")]
    pub density: f64,
    /// the radii of the dots
    #[serde(rename = "radii")]
    pub radii: na::Vector2<f64>,
    /// the distribution type
    #[serde(rename = "distribution")]
    pub distribution: TexturedDotsDistribution,
    /// True if segments should have a constant width ( ignoring pen pressures )
    #[serde(rename = "segment_constant_width")]
    pub segment_constant_width: bool,
}

impl Default for TexturedOptions {
    fn default() -> Self {
        Self {
            seed: None,
            stroke_width: Self::WIDTH_DEFAULT,
            density: Self::DENSITY_DEFAULT,
            stroke_color: Some(Color::BLACK),
            radii: Self::RADII_DEFAULT,
            distribution: TexturedDotsDistribution::default(),
            segment_constant_width: false,
        }
    }
}

impl TexturedOptions {
    /// The default width
    pub const WIDTH_DEFAULT: f64 = 1.0;
    /// Density default
    pub const DENSITY_DEFAULT: f64 = 5.0;
    /// Radii default
    pub const RADII_DEFAULT: na::Vector2<f64> = na::vector![2.0, 0.3];
}

impl From<crate::rnotev0_4::strokes::TexturedOptions> for TexturedOptions {
    fn from(to: crate::rnotev0_4::strokes::TexturedOptions) -> Self {
        let crate::rnotev0_4::strokes::TexturedOptions {
            seed,
            width,
            density,
            stroke_color,
            radii,
            distribution,
        } = to;

        TexturedOptions {
            seed,
            stroke_width: width,
            stroke_color,
            density,
            radii,
            distribution: distribution.into(),
            segment_constant_width: false,
        }
    }
}
