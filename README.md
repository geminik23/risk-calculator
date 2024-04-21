# RiskCalculator

An implementations of the RiskCalculator, a tool designed to compute risk allocations and per-trade risks for trading strategies in both **Python** and **Rust**.

## Formula

The risk for each strategy is calculated using the following simple formula:


```
Expected Weekly Loss = Avg Trades per Week * Loss Rate * (1% of Account Balance)
Risk Allocation = (Strategy's Expected Weekly Loss / Total Expected Weekly Loss) * Max Loss
Per Trade Risk = Risk Allocation / Avg Trades per Week
```

- **Avg Trades per Week**: Average number of trades per week for a strategy.
- **Loss Rate**: Probability of a losing trade (calculated as `1 - Win Rate`).
- **Account Balance**: Total funds available for trading.
- **Max Loss**: Maximum acceptable loss specified for the period.

## JSON Template Example

The JSON file should contain an array of strategy objects. 

Each strategy object must have the following properties:
```json
[
    {
        "name": "Strategy A",
        "avg_trades_week": 10,
        "risk_reward_ratio": 2.5,
        "win_rate": 0.4
    },
    {
        "name": "Strategy B",
        "avg_trades_week": 15,
        "risk_reward_ratio": 1.5,
        "win_rate": 0.5
    },
    ...
]
```

## Usage

### Python

Navigate to the `/py` directory:

```bash
cd py
python main.py strategies.json 50000 2500
```

### Rust

```bash
cargo run -- strategies.json 50000 2500
```
