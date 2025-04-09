use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Direction {
    Buy,
    Sell,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "buy" => Some(Direction::Buy),
            "sell" => Some(Direction::Sell),
            _ => None,
        }
    }

    pub fn from_int(i: i32) -> Option<Self> {
        match i {
            1 => Some(Direction::Buy),
            -1 => Some(Direction::Sell),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Direction::Buy => "buy",
            Direction::Sell => "sell",
        }
    }
}
