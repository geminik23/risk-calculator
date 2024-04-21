from pydantic import BaseModel
from typing import List

class StrategyProvider(BaseModel):
    name: str
    avg_trades_week: int
    risk_reward_ratio: float
    win_rate: float

class RiskAllocationResult(BaseModel):
    name: str
    risk_allocation: float
    per_trade_risk: float

class RiskManager:
    def __init__(self, account_balance: float, max_loss: float):
        self.account_balance = account_balance
        self.max_loss = max_loss

    def allocate_risks(self, strategies: List[StrategyProvider]) -> List[RiskAllocationResult]:
        total_potential_loss = 0
        strategy_losses = []
        for strategy in strategies:
            expected_loss = self._calculate_strategy_risk_allocation(strategy)
            strategy_losses.append(expected_loss)
            total_potential_loss += expected_loss

        results = []
        for strategy, loss in zip(strategies, strategy_losses):
            risk_allocation = (loss / total_potential_loss) * self.max_loss if total_potential_loss else 0
            per_trade_risk = self._calculate_per_trade_risk(strategy.avg_trades_week, risk_allocation)
            results.append(RiskAllocationResult(name=strategy.name, risk_allocation=risk_allocation, per_trade_risk=per_trade_risk))
        return results

    def _calculate_strategy_risk_allocation(self, strategy: StrategyProvider) -> float:
        risk_per_trade = 0.01  # 1% of account balance
        average_loss = risk_per_trade * self.account_balance / strategy.risk_reward_ratio  # risking 1% per trade
        loss_rate = 1 - strategy.win_rate  # probability of a losing trade
        expected_weekly_loss = strategy.avg_trades_week * (loss_rate * average_loss)
        return expected_weekly_loss

    def _calculate_per_trade_risk(self, trades_week: int, risk_allocation: float) -> float:
        return risk_allocation / trades_week if trades_week else 0

