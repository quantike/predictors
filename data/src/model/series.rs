use std::fmt::{Debug, Display};

/// Holds most basic information about a [`Series`].
pub struct Series {
    /// Ticker that identifies this [`Series`].
    pub ticker: String, 

    /// Description of the frequency of the [`Series`]. There is no fixed value set here, but will
    /// be something human-readable like: "weekly", "daily", "one-off".
    ///
    /// TODO: Opportunity to clean up code by enumerating frequencies. Warn developer if attempting
    /// to instantiate a Series with an unrecognized frequency.
    ///
    /// TODO: This might also be an Option<T> field, with a default.
     pub frequency: String,
}

impl Debug for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.frequency, self.ticker)
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ticker)
    }
}

impl Series {
    pub fn new(ticker: String, frequency: String) -> Self {
        Series { ticker, frequency }
    }
}
