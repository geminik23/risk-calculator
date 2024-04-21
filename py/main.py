import argparse
import json
from pydantic import parse_obj_as
from core import StrategyProvider, RiskManager
from typing import List

def main():
    parser = argparse.ArgumentParser(description="Calculate risk allocations for trading strategies.")
    parser.add_argument("json_file", type=str, help="Path to the JSON file containing the strategies")
    parser.add_argument("balance", type=float, help="Account balance")
    parser.add_argument("max_loss", type=float, help="Maximum allowable loss for the period")

    args = parser.parse_args()

    with open(args.json_file, 'r') as file:
        strategies_data = json.load(file)
    strategies = parse_obj_as(List[StrategyProvider], strategies_data)
    
    risk_manager = RiskManager(account_balance=args.balance, max_loss=args.max_loss)
    allocation_results = risk_manager.allocate_risks(strategies)
    
    for result in allocation_results:
        print(f"Strategy {result.name}: Total Weekly Risk Allocation: ${result.risk_allocation:.2f}, Risk Per Trade: ${result.per_trade_risk:.2f}")

if __name__ == "__main__":
    main()

