use std::fmt::Display;
use chrono::{DateTime, Utc};
use super::SubscriptionChannel;


#[derive(Clone, Debug)]
pub struct MarketLifecycles;

impl SubscriptionChannel for MarketLifecycles {
    type Event = Lifecycle;
}

/// Represents the lifecycle of a market, including timestamp for various phases and its final
/// result. 
#[derive(Debug)]
pub struct Lifecycle {
    /// Timestamp for when the market opened.
    pub open_ts: DateTime<Utc>,

    /// Timestamp for when the market is scheduled to close. Will be updated in case of earlt
    /// determination.
    pub close_ts: DateTime<Utc>,

    /// Optional: this field will not exist before the market is determined. Timestamp for when the
    /// market is determined. 
    pub determination_ts: Option<DateTime<Utc>>,
    
    /// Optional: this field will not exist before the market is settled. Timestamp for whe the
    /// market is settled.
    pub settled_ts: Option<DateTime<Utc>>,

    /// Optional: this field will not exist before the market is determined. Result of the market.
    pub result: Option<String>,

    /// Boolean field to indicate if the trading is paused on an open market. This should only be
    /// interpreted for an open market. 
    pub is_deactivated: bool,
}

impl Lifecycle {
    pub fn check_state(&self) -> Option<LifecycleUpdate> {
        let now = Utc::now();

        let (state, ts) = match (self.determination_ts, self.settled_ts, self.is_deactivated, now >= self.close_ts) {
            (None, _, false, _) => (LifecycleState::Opened, self.open_ts),
            (None, _, true, _) => (LifecycleState::Paused, now),
            (None, _, _, true) => (LifecycleState::Closed, self.close_ts),
            (Some(determination_ts), None, _, _) => (LifecycleState::Determined, determination_ts),
            (Some(determination_ts), Some(settled_ts), _, _) => (LifecycleState::Settled, settled_ts),
            // (None, _, true, _) => ifecycleState::Paused, now),
            // (None, _, false, _) => (LifecycleState::Opened, self.open_ts),
            // (Some(determination_ts), None, _, _) => (LifecycleState::Determined, determination_ts),
            // (_, Some(settled_ts), _, _) => (LifecycleState::Settled, settled_ts),
            // (_, _, _, true) => (LifecycleState::Closed, self.close_ts),
            // (_, _, _, _) => return None,
        };

        Some(LifecycleUpdate::new(state, ts, now))
    }
}

pub struct LifecycleUpdate {
    /// Current, inferred state of the Market.
    pub state: LifecycleState,

    /// The associate timestamp for that state.
    pub status_ts: DateTime<Utc>,

    /// The timestamp of the lifecycle update. Usually "now". 
    pub update_ts: DateTime<Utc>,
}

impl LifecycleUpdate {
    pub fn new(state: LifecycleState, status_ts: DateTime<Utc>, update_ts: DateTime<Utc>) -> Self {
        LifecycleUpdate { state, status_ts, update_ts }
    }
}

impl Display for LifecycleUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {} | {}", self.state, self.status_ts, self.update_ts)
    }
}

/// When subscribed to the "market_lifecycle" channel there are several types of lifecycle 
/// updated that can occur to a market. We use [`LifecycleUpdate`] to track and log these
/// events.
#[derive(Debug)]
pub enum LifecycleState {
	/// When a new market is opened. Determined by the presence of a novel "market_ticker" in
	/// the message.
	Opened,

	/// When a market's trading is paused. Should only be relevant for "open" markets.
	/// Determined by the boolean "is_deactivated" flag in the message.
	Paused,

	/// When a market's closed timestamp crosses with no update. Determined by exceeding 
	/// "close_ts" with no updated to the field.
	Closed,

	/// When a market is determined. Determined by the "determined_ts" key in the message.
	/// Additionally, a determination will update the result of the market.
	Determined,

	/// When a market is settled. Determined by the "settled_ts" key in the message.
	Settled,
}

impl Display for LifecycleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Opened => "opened",
                Self::Paused => "paused",
                Self::Closed => "closed",
                Self::Determined => "determined",
                Self::Settled => "settled",
            }
        )
    }
}
