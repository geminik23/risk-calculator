use clap::{Arg, Command};
use risk_calculator::{RiskManager, StrategyProvider};
use serde_json;
use std::fs;

fn main() {
    let matches = Command::new("Risk Manager")
        .version("0.1")
        .author("Jaemin Kim")
        .about("Simple risk calculator for trading strategies")
        .arg(
            Arg::new("json_file")
                .help("JSON file containing the strategies")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("balance")
                .help("Account balance")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("max_loss")
                .help("Maximum allowable loss for week")
                .required(true)
                .index(3),
        )
        .get_matches();

    let json_file = matches.get_one::<String>("json_file").unwrap();
    let balance: f64 = matches
        .get_one::<String>("balance")
        .unwrap()
        .parse::<f64>()
        .expect("The type of balance should be a float");
    let max_loss: f64 = matches
        .get_one::<String>("max_loss")
        .unwrap()
        .parse::<f64>()
        .expect("The type of max loss should be a float");

    let data = fs::read_to_string(json_file).expect("Failed to read file");
    let strategies: Vec<StrategyProvider> =
        serde_json::from_str(&data).expect("Failed to parse JSON");

    let risk_manager = RiskManager::new(balance, max_loss);
    let allocation_results = risk_manager.allocate_risks(&strategies);

    for result in allocation_results {
        println!(
            "{}: Total weekly Risk allocation: ${:.2}, Risk per trade: ${:.2}",
            result.name, result.risk_allocation, result.per_trade_risk
        );
    }
}
