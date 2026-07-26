use crate::models::Asset;

pub fn fetch_assets() -> Vec<Asset> {
    vec![
        Asset {
            symbol: "BTC".into(),
            amount: 0.45,
            price: 65000.0,
        },
        Asset {
            symbol: "ETH".into(),
            amount: 3.0,
            price: 3400.0,
        },
    ]
}
