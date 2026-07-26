use crate::storage;

pub fn export_csv() {
    let snapshots = storage::load();

    let mut writer = csv::Writer::from_path("portfolio.csv").unwrap();

    writer
        .write_record(["Date", "Portfolio Value"])
        .unwrap();

    for s in snapshots {
        writer
            .write_record([s.date, s.total_value.to_string()])
            .unwrap();
    }

    writer.flush().unwrap();

    println!("CSV exported.");
}
