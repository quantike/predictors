use std::{borrow::Cow, fmt::{Debug, Display}};

use market::Market;

/// [`Market`] related data structures.
///
/// i.e. [`Market`], [`MarketType`]
/// 
/// Note: Eventually, we need to create specific data structures and implementations for each
/// [`MarketType`]; [`MarketType::Binary`] and [`MarketType::Scalar`].
pub mod market;

/// [`Event`] related data structures.
///
/// i.e. [`Event`]
///
/// Note: We may want to include some MECENT logic at this level for groups of [`Market`]s that
/// belong to the same [`Event`].
pub mod event;

/// [`Series`] related data structures.
///
/// i.e. [`Series`]
///
/// Note: We might want to explicitly enumerate supported [`Series`] frequencies with an enum (e.g.
/// "daily", "weekly", etc.). This would require a bit of data mining to figure out. 
pub mod series;

/// Represents a unique combination of an [`Exchange`] and [`Market`], which make up a
/// [`PredictionMarket`]. 
pub struct PredictionMarket<MarketId = Market> {
    pub exchange: Exchange,
    pub market: MarketId,
}

impl <E, M, MarketId> From<(E, M)> for PredictionMarket<MarketId>
where
    E: Into<Exchange>,
    M: Into<MarketId>,
{
    fn from((exchange, market): (E, M)) -> Self {
        Self::new(exchange, market)
    }
}

impl<MarketId> PredictionMarket<MarketId> {
    pub fn new<E, M>(exchange: E, market: M) -> Self 
    where 
        E: Into<Exchange>,
        M: Into<MarketId>,
    {
        Self { exchange: exchange.into(),  market: market.into() }    
    }
}

/// Identification for a [`PredictionMarket`] which is a unique `String` identifier which represents a [`Market`] being traded on and [`Exchange`].
pub struct PredictionMarketId(pub String);

impl Debug for PredictionMarketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for PredictionMarketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Predictors representation of an [`Exchange`]'s name.
///
/// e.g. Exchange("kalshi")
pub struct Exchange(Cow<'static, str>);

impl<E> From<E> for Exchange
where 
    E: Into<Cow<'static, str>>,
{
    fn from(exchange: E) -> Self {
        Exchange(exchange.into())
    }
}

impl Debug for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

