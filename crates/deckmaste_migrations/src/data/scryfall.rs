use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Catalog {
    pub object: String,
    pub uri: String,
    pub total_values: u32,
    pub data: Vec<String>,
}
