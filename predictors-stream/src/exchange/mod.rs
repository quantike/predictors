use std::fmt::Display;

/// Unique identifier for an exchange server.
///
/// Note: Only one exchange for now.
pub enum ExchangeId {
    /// ID: "KAL"
    Kalshi,
}

impl Display for ExchangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ExchangeId {
    /// Return the `&str` representation for this [`ExchangeId`]. All exchanges use a three-letter
    /// shorthand string.
    ///
    /// Note: This may change.
    pub fn as_str(&self) -> &'static str {
        match self {
            ExchangeId::Kalshi => "KAL",
        }
    }
}
