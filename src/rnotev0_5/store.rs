use serde::{Deserialize, Serialize};

use super::stroke::Stroke;
pub use crate::rnotev0_4::{ChronoComponent, SelectionComponent, TrashComponent};
use crate::slot::{SerdeSlot, SlotMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, rename = "history_entry")]
pub struct HistoryEntry {
    #[serde(rename = "stroke_components")]
    stroke_components: SlotMap<Stroke>,
    #[serde(rename = "trash_components")]
    trash_components: SlotMap<TrashComponent>,
    #[serde(rename = "selection_components")]
    selection_components: SlotMap<SelectionComponent>,
    #[serde(rename = "chrono_components")]
    chrono_components: SlotMap<ChronoComponent>,

    #[serde(rename = "chrono_counter")]
    chrono_counter: u32,
}

impl Default for HistoryEntry {
    fn default() -> Self {
        Self {
            stroke_components: Vec::new(),
            trash_components: Vec::new(),
            selection_components: Vec::new(),
            chrono_components: Vec::new(),
            chrono_counter: 0,
        }
    }
}

// the store snapshot, used when saving the store to a file.
pub type StoreSnapshot = HistoryEntry;

impl From<crate::rnotev0_4::StrokesState> for HistoryEntry {
    fn from(ss: crate::rnotev0_4::StrokesState) -> Self {
        Self {
            stroke_components: ss
                .strokes
                .into_iter()
                .filter_map(|s| {
                    let value = if let Some(val) = s.value {
                        Some(Into::<Option<Stroke>>::into(val)?)
                    } else {
                        None
                    };

                    Some(SerdeSlot {
                        value,
                        version: s.version,
                    })
                })
                .collect(),
            trash_components: ss.trash_components,
            selection_components: ss.selection_components,
            chrono_components: ss.chrono_components,
            chrono_counter: ss.chrono_counter,
        }
    }
}
