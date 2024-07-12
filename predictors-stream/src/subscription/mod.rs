use std::fmt::Debug;

/// [`SubChannel::MarketLifecycles`] [`SubscriptionChannel`]s and the associate output data models.
pub mod lifecycle;


/// Defines the type of a [`Subscription`], and the output [`Self::Event`] that it yields.
pub trait SubscriptionChannel
where 
    Self: Debug + Clone,
{
    type Event: Debug;
}

/// [`Subscription`] used to subscribe to a [`SubscriptionChannel`] for a given exchange and
/// [`Market`]
pub struct Subscription<Exchange, Market, Channel> {
    pub exchange: Exchange,
    pub market: Market,
    pub channel: Channel,
}

/// Valid WebSocket channels. We use [`SubChannel`] to create feeds of logically related messages.
/// Enumerated below are the provisional channels supported, which are subject to change.
///
/// *Observation* Channels are standardized with a singular name form (e.g. `OrderbookDelta` will
/// result in messages of type `snapshot` and `delta`.).
pub enum SubChannel {
    /// A complete view of the order book's aggregated price levels on a given market and all
    /// further updates to it. 
    OrderbookDeltas,

    /// The list price ticker for a given market.
    ///
    /// No snapshot required. The exchange just sends the last price on the market when the price
    /// changes. On active markets, when the price changes a few times per second, only the most
    /// recent change is send for that second.
    ///
    /// TODO: Does this mean that certain fields in the channel messages are ~optional~ or does
    /// this mean that the minimum update frequency for all fields is once per second.
    Tickers,

    /// Update the client with the most recent trades that occur in the markets that the client is
    /// subscribed to. 
    ///
    /// The subscription process is similar to [`SubChannel::Tickers`], the client specifies the
    /// markets they are interested in subscribing to and the server with start sending trade data
    /// messages.
    Trades,

    /// Update the client with the most recent fills in the market that client is subscribed to.
    Fills,

    /// Update the client with new market lifecycle events of the following types:
    /// [`LifecycleUpdate::Opened`], [`LifecycleUpdate::Paused`], [`LifecycleUpdate::Closed`],
    /// [`LifecycleUpdate::Determined`], and [`LifecycleUpdate::Settled`] with the corresponding
    /// details for each event.
    MarketLifecycles,
}
