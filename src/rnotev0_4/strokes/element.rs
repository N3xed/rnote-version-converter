use serde::{Deserialize, Serialize};

use super::inputdata::InputData;

// Represents a single Stroke Element
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "element")]
pub struct Element {
    #[serde(rename = "inputdata")]
    pub inputdata: InputData,
    #[serde(rename = "timestamp")]
    pub timestamp: Option<Timestamp>,
}

impl Default for Element {
    fn default() -> Self {
        Self { inputdata: Default::default(), timestamp: Default::default() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Timestamp(String);