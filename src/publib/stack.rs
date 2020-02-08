use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum ProjectStack {
    #[serde(rename = "rust")]
    Rust,

    #[serde(rename = "go")]
    Go,
}

impl Eq for ProjectStack {}
