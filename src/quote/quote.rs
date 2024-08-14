use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub open: Option<f32>,
    pub close: Option<f32>,
    pub high: Option<f32>,
    pub low: Option<f32>,
    pub date: String,
}

impl Quote {
    pub fn is_empty(&self) -> bool {
        self.open.is_none() && self.close.is_none() && self.high.is_none() && self.low.is_none()
    }
}