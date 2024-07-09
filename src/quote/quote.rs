use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub(crate) open: Option<f32>,
    pub(crate) close: Option<f32>,
    pub(crate) high: Option<f32>,
    pub(crate) low: Option<f32>,
    pub(crate) date: String,
}