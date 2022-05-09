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
        let mut remove_list = vec![];
        let stroke_components = ss
            .strokes
            .into_iter()
            .enumerate()
            .filter_map(|(i, s)| {
                let value = if let Some(val) = s.value {
                    match TryInto::<Stroke>::try_into(val) {
                        Ok(val) => Some(val),
                        Err(err) => {
                            eprintln!("{:#?}", err);
                            remove_list.push(i);
                            return None;
                        }
                    }
                } else {
                    None
                };

                Some(SerdeSlot {
                    value,
                    version: s.version,
                })
            })
            .collect();

        fn remove_from_vec<T>(index_list: &Vec<usize>, mut vec: Vec<T>) -> Vec<T> {
            for index in index_list.iter().rev() {
                vec.remove(*index);
            }
            vec
        }

        Self {
            stroke_components,
            trash_components: remove_from_vec(&remove_list, ss.trash_components),
            selection_components: remove_from_vec(&remove_list, ss.selection_components),
            chrono_components: remove_from_vec(&remove_list, ss.chrono_components),
            chrono_counter: ss.chrono_counter,
        }
    }
}
