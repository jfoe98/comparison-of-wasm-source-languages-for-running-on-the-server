use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Metric {
    pub language: String,
    pub name: String,
    pub context: String,
    pub value: String,
}

impl Metric {
    pub fn new(language: String, name: String, context: String, value: String) -> Metric {
        Metric {
            language,
            name,
            context,
            value,
        }
    }
}