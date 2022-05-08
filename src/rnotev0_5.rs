use serde::{Deserialize, Serialize};

pub use crate::rnotev0_4::transform::Transform;
use crate::rnotev0_4::Background;
pub use crate::rnotev0_4::{Color, Orientation};

pub mod penpath;
pub mod shapes;
pub mod store;
pub mod stroke;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "rnotefile_wrapper")]
pub struct RnotefileWrapper {
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "data")]
    data: RnotefileMaj0Min5,
}

impl RnotefileWrapper {
    pub const VERSION: &'static str = "0.5.1";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "rnotefile_maj0_min5")]
pub struct RnotefileMaj0Min5 {
    /// the document
    #[serde(rename = "document", alias = "sheet")]
    pub document: Document,
    /// A snapshot of the store
    #[serde(rename = "store_snapshot")]
    pub store_snapshot: store::StoreSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "document")]
pub struct Document {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
    #[serde(rename = "format")]
    pub format: Format,
    #[serde(rename = "background")]
    pub background: Background,
    #[serde(rename = "layout", alias = "expand_mode")]
    layout: Layout,
}

impl Default for Document {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: Format::default().width,
            height: Format::default().height,
            format: Format::default(),
            background: Background::default(),
            layout: Layout::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(default, rename = "format")]
pub struct Format {
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
    #[serde(rename = "dpi")]
    pub dpi: f64,
    #[serde(rename = "orientation")]
    pub orientation: Orientation,
    #[serde(rename = "border_color", default)]
    pub border_color: Color,
    #[serde(rename = "show_borders")]
    pub show_borders: bool,
}

impl Default for Format {
    fn default() -> Self {
        Self {
            width: Self::WIDTH_DEFAULT,
            height: Self::HEIGHT_DEFAULT,
            dpi: Self::DPI_DEFAULT,
            orientation: Orientation::default(),
            border_color: Color::RED,
            show_borders: true,
        }
    }
}

impl From<crate::rnotev0_4::Format> for Format {
    fn from(format: crate::rnotev0_4::Format) -> Self {
        Self {
            width: format.width,
            height: format.height,
            dpi: format.dpi,
            orientation: format.orientation,
            ..Default::default()
        }
    }
}

impl Format {
    pub const WIDTH_MIN: f64 = 1.0;
    pub const WIDTH_MAX: f64 = 30000.0;
    pub const WIDTH_DEFAULT: f64 = 1123.0;

    pub const HEIGHT_MIN: f64 = 1.0;
    pub const HEIGHT_MAX: f64 = 30000.0;
    pub const HEIGHT_DEFAULT: f64 = 1587.0;

    pub const DPI_MIN: f64 = 1.0;
    pub const DPI_MAX: f64 = 5000.0;
    pub const DPI_DEFAULT: f64 = 96.0;
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename = "layout")]
pub enum Layout {
    #[serde(rename = "fixed_size")]
    FixedSize,
    #[serde(rename = "continuous_vertical", alias = "endless_vertical")]
    ContinuousVertical,
    #[serde(rename = "infinite")]
    Infinite,
}

impl Default for Layout {
    fn default() -> Self {
        Self::Infinite
    }
}

pub mod base64 {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    /// Serialize a Vec<u8> as base64 encoded
    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::encode(v);
        String::serialize(&base64, s)
    }

    /// Deserialize base64 encoded Vec<u8>
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::decode(base64.as_bytes()).map_err(|e| serde::de::Error::custom(e))
    }
}

impl From<crate::rnotev0_4::Sheet> for RnotefileWrapper {
    fn from(sheet: crate::rnotev0_4::Sheet) -> Self {
        let crate::rnotev0_4::Sheet {
            x,
            y,
            width,
            height,
            strokes_state,
            format,
            background,
            ..
        } = sheet;

        let document = Document {
            x,
            y,
            width,
            height,
            format: format.into(),
            background,
            ..Default::default()
        };

        let data = RnotefileMaj0Min5 {
            document,
            store_snapshot: strokes_state.into()
        };
        Self {
            version: Self::VERSION.to_owned(),
            data,
        }
    }
}
