use crate::models::{Asset, Snapshot};
use crate::storage;

pub fn save_snapshot() {
    let assets = vec![
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
    ];

    let total = assets.iter()
        .map(|a| a.amount * a.price)
        .sum();

    let snapshot = Snapshot {
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
        total_value: total,
        assets,
    };

    storage::save(snapshot);

    println!("Snapshot saved.");
}
