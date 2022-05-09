use serde::{Deserialize, Serialize};

pub mod strokesstate;
pub use strokesstate::*;
pub mod curves;
pub mod geometry;
pub mod roughoptions;
pub mod shapes;
pub mod strokes;
pub mod transform;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename = "sheet")]
pub struct Sheet {
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
    #[serde(rename = "strokes_state")]
    pub strokes_state: StrokesState,
    #[serde(rename = "format")]
    pub format: Format,
    #[serde(rename = "background")]
    pub background: Background,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default, rename = "width")]
pub struct Format {
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
    #[serde(rename = "dpi")]
    pub dpi: f64,
    #[serde(rename = "orientation")]
    pub orientation: Orientation,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            dpi: Self::DPI_DEFAULT,
            orientation: Orientation::Portrait,
        }
    }
}

impl Format {
    pub const DPI_DEFAULT: f64 = 96.0;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
#[serde(rename = "orientation")]
pub enum Orientation {
    //#[enum_value(name = "Portrait", nick = "portrait")]
    #[serde(rename = "portrait")]
    Portrait = 0,
    //#[enum_value(name = "Landscape", nick = "landscape")]
    #[serde(rename = "landscape")]
    Landscape,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Portrait
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "background")]
pub struct Background {
    #[serde(rename = "color")]
    pub color: Color,
    #[serde(rename = "pattern")]
    pub pattern: PatternStyle,
    #[serde(rename = "pattern_size")]
    pub pattern_size: na::Vector2<f64>,
    #[serde(rename = "pattern_color")]
    pub pattern_color: Color,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            pattern: PatternStyle::Dots,
            pattern_size: na::vector![32.0, 32.0],
            pattern_color: Color {
                r: 0.8,
                g: 0.9,
                b: 1.0,
                a: 1.0,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[repr(u32)]
#[serde(rename = "pattern_style")]
pub enum PatternStyle {
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "lines")]
    Lines,
    #[serde(rename = "grid")]
    Grid,
    #[serde(rename = "dots")]
    Dots,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "color")]
pub struct Color {
    #[serde(rename = "r")]
    pub r: f64, // between 0.0 and 1.0
    #[serde(rename = "g")]
    pub g: f64, // between 0.0 and 1.0
    #[serde(rename = "b")]
    pub b: f64, // between 0.0 and 1.0
    #[serde(rename = "a")]
    pub a: f64, // between 0.0 and 1.0
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Color {
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}
