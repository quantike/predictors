use std::fmt::Display;


/// Identifies the type of market, which affects its payout structure.
pub enum MarketType {
    /// Every binary market has two sides, YES and NO. If the market's "payout criterion" is
    /// satisfied, it pays out the notional value to holders of YES. Otherwise, it pays out the
    /// notional holders of NO.
    Binary,

    /// Every scalar market has two sides, LONG and SHORT (although these might be referred to as
    /// YES/NO in some API endpoints). At settlement, each contract's notional value is split
    /// between LONG and SHORT as described by the rules.
    Scalar,
}

impl Default for MarketType {
    fn default() -> Self {
        Self::Binary
    }
}

impl Display for MarketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}", 
            match self {
                MarketType::Binary => "binary".to_string(),
                MarketType::Scalar => "scalar".to_string(),
            }
        )
    }
}

/// Identifies the most basic information about a market.
pub struct Market<'a> {
    ticker: String,
    market_type: MarketType,
    event: &a Event,
}
