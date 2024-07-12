use super::SubscriptionChannel;


#[derive(Clone, Debug)]
pub struct MarketLifecycles;

impl SubscriptionChannel for MarketLifecycles {
    type Event = Lifecycle;
}

#[derive(Debug)]
pub struct Lifecycle {
    pub open_ts: i64,
    pub close_ts: i64,
    pub determination_ts: i64,
    pub settled_ts: i64,
    pub result: String,
    pub is_deactivated: bool,
}

/// When subscribed to the "market_lifecycle" channel there are several types of lifecycle 
/// updated that can occur to a market. We use [`LifecycleUpdate`] to track and log these
/// events.
#[derive(Debug)]
pub enum LifecycleUpdate {
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
