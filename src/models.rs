use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Asset {
    pub symbol: String,
    pub amount: f64,
    pub price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub date: String,
    pub total_value: f64,
    pub assets: Vec<Asset>,
}
