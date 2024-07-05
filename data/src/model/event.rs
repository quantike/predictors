use std::fmt::{Debug, Display};
use crate::model::series::Series;

/// Holds the most basic information about an [`Event`].
pub struct Event<'a> {
    /// Unique identifier for [`Event`]s.
    ticker: String,

    /// If true then the [`Event`] is mutually exclusive.
    mutually_exclusive: bool,

    /// The reference to the parent [`Series`] of this [`Event`].
    ///
    /// This field holds a reference to the [`Series`] instance that represents the parent-series
    /// to which this [`Event`] belongs. It allows accessing properties and methods of the
    /// [`Series`] from within an [`Event`].
    series: &'a Series,
}

impl<'a> Debug for Event<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MECENT {}", self.ticker)
    }
}

impl<'a> Display for Event<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ticker)
    }
}

impl<'a> Event<'a> {
    pub fn new(ticker: &str, mutually_exclusive: bool, series: &'a Series) -> Self {
        Event { 
            ticker: ticker.to_string(), 
            mutually_exclusive, 
            series,
        }
    }
}
