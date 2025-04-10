use serde::{Deserialize, Serialize};

/// Direction of the trade
/// Future - could support long/short or other types of trades
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Buy,
    Sell,
}

impl Direction {
    /// Case-insensitive conversion from string to Direction
    /// Accepts "buy" or "sell", "BUY" or "SELL"
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "buy" => Some(Direction::Buy),
            "sell" => Some(Direction::Sell),
            _ => None,
        }
    }

    /// Conversion from integer to Direction (>0 Buy, <0 Sell)
    pub fn from_int(i: i32) -> Option<Self> {
        match i {
            i if i > 0 => Some(Direction::Buy),
            i if i < 0 => Some(Direction::Sell),
            _ => None,
        }
    }

    /// Conversion from Direction to string representation ("buy" or "sell")
    pub fn to_str(&self) -> &str {
        match self {
            Direction::Buy => "buy",
            Direction::Sell => "sell",
        }
    }
}
