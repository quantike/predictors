use std::fmt::Display;

use super::event::Event;


/// Identifies the type of market, which affects its payout structure.
pub enum MarketType {
    /// Every [`MarketType::Binary`] has two sides, YES and NO. If the market's "payout criterion" is
    /// satisfied, it pays out the notional value to holders of YES. Otherwise, it pays out the
    /// notional holders of NO.
    Binary,

    /// Every [`MarketType::Scalar`] has two sides, LONG and SHORT (although these might be referred to as
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
                MarketType::Binary => "BINARY".to_string(),
                MarketType::Scalar => "SCALAR".to_string(),
            }
        )
    }
}

/// Identifies the most basic information about a market.
pub struct Market<'a> {
    /// Unique identifier for a [`Market`].
    pub ticker: String,

    /// Identifies the type of [`Market`], which affects its payout and structure. Uses the enum
    /// [`MarketType`] to designate the intended type.
    ///
    /// The two types are [`MarketType::Binary`] or [`MarketType::Scalar`].
    ///
    /// TODO: An opportunity for improvement would be to use traits to force impls for new
    /// [`Market`]s depending on the [`MarketType`]. For example, [`MarketType::Scalar`] would
    /// affect a some type of [`Market::payout_structure()`] function.
    pub market_type: MarketType,

    /// Reference to the [`Market`]'s parent [`Event`].
    event: &'a Event<'a>,
}

impl<'a> Display for Market<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.ticker, self.market_type)
    }
}

impl<'a> Market<'a> {
    pub fn new(ticker: &str, market_type: MarketType, event: &'a Event) -> Self {
        Market { ticker: ticker.to_string(), market_type, event }
    }

    pub fn event_exclusivity(&self) -> &bool {
        &self.event.mutually_exclusive
    }

    pub fn payout_structure(&self) -> Option<f64> {
        unimplemented!()
    }
}
