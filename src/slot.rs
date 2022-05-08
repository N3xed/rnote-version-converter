use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeSlot<T> {
    pub value: Option<T>,
    pub version: u32
}

pub type SlotMap<T> = Vec<SerdeSlot<T>>;