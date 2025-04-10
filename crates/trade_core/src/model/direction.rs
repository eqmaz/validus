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

// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =
// Unit tests for direction.rs
// = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = =

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_from_str_valid_inputs() {
        assert_eq!(Direction::from_str("buy"), Some(Direction::Buy));
        assert_eq!(Direction::from_str("BUY"), Some(Direction::Buy));
        assert_eq!(Direction::from_str("BuY"), Some(Direction::Buy));
        assert_eq!(Direction::from_str("sell"), Some(Direction::Sell));
        assert_eq!(Direction::from_str("SELL"), Some(Direction::Sell));
        assert_eq!(Direction::from_str("sElL"), Some(Direction::Sell));
    }

    #[test]
    fn test_from_str_invalid_input() {
        assert_eq!(Direction::from_str("hold"), None);
        assert_eq!(Direction::from_str(""), None);
        assert_eq!(Direction::from_str("buyy"), None);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(Direction::from_int(10), Some(Direction::Buy));
        assert_eq!(Direction::from_int(-5), Some(Direction::Sell));
        assert_eq!(Direction::from_int(0), None);
    }

    #[test]
    fn test_to_str() {
        let buy = Direction::Buy;
        let sell = Direction::Sell;
        assert_eq!(buy.to_str(), "buy");
        assert_eq!(sell.to_str(), "sell");
    }

    #[test]
    fn test_serde_serialization() {
        let buy = Direction::Buy;
        let serialized = serde_json::to_string(&buy).unwrap();
        assert_eq!(serialized, "\"Buy\"");

        let deserialized: Direction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Direction::Buy);
    }

    #[test]
    fn test_serde_deserialization_invalid() {
        let invalid_json = "\"Hold\"";
        let result: Result<Direction, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }
}
