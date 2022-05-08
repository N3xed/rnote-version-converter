pub mod bitmapimage;
pub mod brush;
pub mod brushstroke;
pub mod element;
pub mod inputdata;
pub mod shapestroke;
pub mod vectorimage;

use serde::{Deserialize, Serialize};

use super::Color;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "smoothoptions")]
pub struct SmoothOptions {
    /// An optional seed to generate reproducable strokes
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "stroke_color")]
    pub stroke_color: Option<Color>,
    #[serde(rename = "fill_color")]
    pub fill_color: Option<Color>,
}

impl Default for SmoothOptions {
    fn default() -> Self {
        Self {
            seed: None,
            width: Self::WIDTH_DEFAULT,
            stroke_color: Some(Self::COLOR_DEFAULT),
            fill_color: None,
        }
    }
}

impl SmoothOptions {
    /// The default width
    pub const WIDTH_DEFAULT: f64 = 1.0;
    /// The min width
    pub const WIDTH_MIN: f64 = 0.1;
    /// The max width
    pub const WIDTH_MAX: f64 = 1000.0;
    /// The default color
    pub const COLOR_DEFAULT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
}

/// The distribution for the spread of dots across the width of the textured stroke
#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
pub enum TexturedDotsDistribution {
    Uniform = 0,
    Normal,
    Exponential,
    ReverseExponential,
}

impl Default for TexturedDotsDistribution {
    fn default() -> Self {
        Self::Normal
    }
}

/// The Options of how a textured shape should look

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "textured_options")]
pub struct TexturedOptions {
    /// An optional seed to generate reproducable strokes
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    /// The width
    #[serde(rename = "width")]
    pub width: f64,
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
}

impl Default for TexturedOptions {
    fn default() -> Self {
        Self {
            seed: None,
            width: Self::WIDTH_DEFAULT,
            density: Self::DENSITY_DEFAULT,
            stroke_color: Some(Self::COLOR_DEFAULT),
            radii: Self::RADII_DEFAULT,
            distribution: TexturedDotsDistribution::default(),
        }
    }
}

impl TexturedOptions {
    /// The default width
    pub const WIDTH_DEFAULT: f64 = 1.0;
    /// The default color
    pub const COLOR_DEFAULT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    /// Density default
    pub const DENSITY_DEFAULT: f64 = 5.0;
    /// Radii default
    pub const RADII_DEFAULT: na::Vector2<f64> = na::vector![2.0, 0.3];
}
