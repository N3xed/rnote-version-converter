use serde::{Serialize, Deserialize};

use self::{smoothoptions::SmoothOptions, roughoptions::RoughOptions, texturedoptions::TexturedOptions};

pub mod roughoptions;
pub mod smoothoptions;
pub mod textureddotsdistribution;
pub mod texturedoptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A style choice holding the style options inside its variants
#[serde(rename = "style")]
pub enum Style {
    /// A smooth style
    #[serde(rename = "smooth")]
    Smooth(SmoothOptions),
    /// A rough style
    #[serde(rename = "rough")]
    Rough(RoughOptions),
    /// A textured style
    #[serde(rename = "textured")]
    Textured(TexturedOptions),
}

impl Default for Style {
    fn default() -> Self {
        Self::Smooth(SmoothOptions::default())
    }
}

impl Style {
    /// returns the stroke width. available on all styles
    pub fn stroke_width(&self) -> f64 {
        match self {
            Style::Smooth(options) => options.stroke_width,
            Style::Rough(options) => options.stroke_width,
            Style::Textured(options) => options.stroke_width,
        }
    }
}