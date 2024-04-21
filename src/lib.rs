use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct StrategyProvider {
    pub name: String,
    pub avg_trades_week: usize,
    pub risk_reward_ratio: f64,
    pub win_rate: f64,
}

#[derive(Debug, Serialize)]
pub struct RiskAllocationResult {
    pub name: String,
    pub risk_allocation: f64,
    pub per_trade_risk: f64,
}

pub struct RiskManager {
    account_balance: f64,
    max_loss: f64,
}

impl RiskManager {
    pub fn new(account_balance: f64, max_loss: f64) -> Self {
        Self {
            account_balance,
            max_loss,
        }
    }

    //
    // Getters and Setters
    pub fn set_account_balance(&mut self, account_balance: f64) {
        self.account_balance = account_balance;
    }

    pub fn set_max_loss(&mut self, max_loss: f64) {
        self.max_loss = max_loss;
    }

    pub fn account_balance(&self) -> f64 {
        self.account_balance
    }

    pub fn max_loss(&self) -> f64 {
        self.max_loss
    }

    //
    pub fn allocate_risks(&self, strategies: &[StrategyProvider]) -> Vec<RiskAllocationResult> {
        let total_potential_loss: f64 = strategies
            .iter()
            .map(|strategy| self.calculate_strategy_risk_allocation(strategy))
            .sum();

        strategies
            .iter()
            .map(|strategy| {
                // !! duplicate calculation
                let expected_loss = self.calculate_strategy_risk_allocation(strategy);
                let risk_allocation = if total_potential_loss > 0.0 {
                    (expected_loss / total_potential_loss) * self.max_loss
                } else {
                    0.0
                };
                let per_trade_risk =
                    self.calculate_per_trade_risk(strategy.avg_trades_week, risk_allocation);
                RiskAllocationResult {
                    name: strategy.name.clone(),
                    risk_allocation,
                    per_trade_risk,
                }
            })
            .collect()
    }

    fn calculate_strategy_risk_allocation(&self, strategy: &StrategyProvider) -> f64 {
        let risk_per_trade = 0.01 * self.account_balance;
        let average_loss = risk_per_trade;
        let loss_rate = 1.0 - strategy.win_rate;
        let expected_weekly_loss = strategy.avg_trades_week as f64 * (loss_rate * average_loss);
        expected_weekly_loss
    }

    fn calculate_per_trade_risk(&self, trades_week: usize, risk_allocation: f64) -> f64 {
        if trades_week > 0 {
            risk_allocation / trades_week as f64
        } else {
            0.0
        }
    }
}
