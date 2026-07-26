mod cli;
mod export;
mod models;
mod portfolio;
mod services;
mod storage;
mod utils;

use clap::Parser;

fn main() {
    let args = cli::Cli::parse();

    match args.command.as_str() {
        "snapshot" => portfolio::save_snapshot(),
        "export" => export::export_csv(),
        _ => println!("Unknown command"),
    }
}
