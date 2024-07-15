use std::fmt::Display;
use crate::model::series::Series;

/// Holds the most basic information about an [`Event`].
pub struct Event {
    /// Unique identifier for [`Event`]s.
    pub ticker: String,

    /// If true then the [`Event`] is mutually exclusive.
    pub mutually_exclusive: bool,

    /// The reference to the parent [`Series`] of this [`Event`].
    series: Series,
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.ticker, self.mutually_exclusive)
    }
}

impl Event {
    pub fn new(ticker: &str, mutually_exclusive: bool, series: Series) -> Self {
        // possibly as a check to ensure event ticker inherits from series ticker?
        Event { 
            ticker: ticker.to_string(), 
            mutually_exclusive, 
            series,
        }
    }

    /// Returns the frequency of the parent [`Series`] for this [`Event`].
    ///
    /// Accesses a reference to the `frequency` field of the parent [`Series`] instance to which
    /// this [`Event`] belongs. Allows querying the frequency information directly from the
    /// [`Event`].
    pub fn series_frequency(&self) -> &str {
        &self.series.frequency
    }
}
