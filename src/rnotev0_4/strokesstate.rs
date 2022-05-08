use serde::{Deserialize, Serialize};

use crate::slot::SlotMap;

use super::strokes::*;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename = "strokes_state")]
pub struct StrokesState {
    // Components
    #[serde(rename = "strokes")]
    pub strokes: SlotMap<StrokeStyle>,
    #[serde(rename = "trash_components")]
    pub trash_components: SlotMap<TrashComponent>,
    #[serde(rename = "selection_components")]
    pub selection_components: SlotMap<SelectionComponent>,
    #[serde(rename = "chrono_components")]
    pub chrono_components: SlotMap<ChronoComponent>,
    #[serde(rename = "render_components")]
    pub render_components: SlotMap<RenderComponent>,

    // Other state
    /// value is equal chrono_component of the newest inserted or modified stroke.
    #[serde(rename = "chrono_counter")]
    pub chrono_counter: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "strokestyle")]
pub enum StrokeStyle {
    #[serde(rename = "brushstroke")]
    BrushStroke(brushstroke::BrushStroke),
    #[serde(rename = "shapestroke")]
    ShapeStroke(shapestroke::ShapeStroke),
    #[serde(rename = "vectorimage")]
    VectorImage(vectorimage::VectorImage),
    #[serde(rename = "bitmapimage")]
    BitmapImage(bitmapimage::BitmapImage),
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self::BrushStroke(brushstroke::BrushStroke::default())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "trash_component")]
pub struct TrashComponent {
    #[serde(rename = "trashed")]
    pub trashed: bool,
}

impl Default for TrashComponent {
    fn default() -> Self {
        Self { trashed: false }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, rename = "selection_component")]
pub struct SelectionComponent {
    #[serde(default, rename = "selected")]
    pub selected: bool,
}

impl Default for SelectionComponent {
    fn default() -> Self {
        Self { selected: false }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(default, rename = "chrono_component")]
pub struct ChronoComponent {
    #[serde(rename = "t")]
    pub t: u32,
}

impl Default for ChronoComponent {
    fn default() -> Self {
        Self { t: 0 }
    }
}

impl ChronoComponent {
    pub fn new(t: u32) -> Self {
        Self { t }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "render_component")]
pub struct RenderComponent {
    #[serde(rename = "render")]
    pub render: bool,
}

impl Default for RenderComponent {
    fn default() -> Self {
        Self { render: true }
    }
}
