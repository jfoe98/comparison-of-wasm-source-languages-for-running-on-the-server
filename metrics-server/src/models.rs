use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Metric {
    pub language: String,
    pub name: String,
    pub context: String,
    pub value: String,
}