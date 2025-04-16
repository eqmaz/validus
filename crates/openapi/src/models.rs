#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApproveTradePathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct BookTradePathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CancelTradePathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetTradeDetailsPathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetTradeHistoryPathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetTradeStatusPathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ListTradesQueryParams {
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SendTradePathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SubmitTradePathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeDiffPathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeDiffQueryParams {
    #[serde(rename = "v1")]
    pub v1: i32,
    #[serde(rename = "v2")]
    pub v2: i32,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UpdateTradePathParams {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HelloResponse {
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl HelloResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> HelloResponse {
        HelloResponse { message: None }
    }
}

/// Converts the HelloResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for HelloResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![self.message.as_ref().map(|message| ["message".to_string(), message.to_string()].join(","))];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HelloResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HelloResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing HelloResponse".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep
                        .message
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing HelloResponse".to_string()),
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HelloResponse { message: intermediate_rep.message.into_iter().next() })
    }
}

// Methods for converting between header::IntoHeaderValue<HelloResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<HelloResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<HelloResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for HelloResponse - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<HelloResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <HelloResponse as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into HelloResponse - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeCreateRequest {
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<models::TradeDetails>,
}

impl TradeCreateRequest {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> TradeCreateRequest {
        TradeCreateRequest { user_id: None, details: None }
    }
}

/// Converts the TradeCreateRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TradeCreateRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.user_id.as_ref().map(|user_id| ["userId".to_string(), user_id.to_string()].join(",")),
            // Skipping details in query parameter serialization
        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeCreateRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeCreateRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<String>,
            pub details: Vec<models::TradeDetails>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeCreateRequest".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "userId" => intermediate_rep
                        .user_id
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "details" => intermediate_rep
                        .details
                        .push(<models::TradeDetails as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => {
                        return std::result::Result::Err("Unexpected key while parsing TradeCreateRequest".to_string())
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeCreateRequest {
            user_id: intermediate_rep.user_id.into_iter().next(),
            details: intermediate_rep.details.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeCreateRequest> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeCreateRequest>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeCreateRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TradeCreateRequest - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TradeCreateRequest> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <TradeCreateRequest as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into TradeCreateRequest - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeCreateResponse {
    #[serde(rename = "tradeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
}

impl TradeCreateResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> TradeCreateResponse {
        TradeCreateResponse { trade_id: None }
    }
}

/// Converts the TradeCreateResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TradeCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![self.trade_id.as_ref().map(|trade_id| ["tradeId".to_string(), trade_id.to_string()].join(","))];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeCreateResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeCreateResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trade_id: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeCreateResponse".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tradeId" => intermediate_rep
                        .trade_id
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => {
                        return std::result::Result::Err("Unexpected key while parsing TradeCreateResponse".to_string())
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeCreateResponse { trade_id: intermediate_rep.trade_id.into_iter().next() })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeCreateResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeCreateResponse>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeCreateResponse>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TradeCreateResponse - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TradeCreateResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <TradeCreateResponse as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into TradeCreateResponse - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeDetails {
    #[serde(rename = "trading_entity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading_entity: Option<String>,

    #[serde(rename = "counterparty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,

    /// Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "direction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,

    #[serde(rename = "notional_currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_currency: Option<String>,

    #[serde(rename = "notional_amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional_amount: Option<f64>,

    #[serde(rename = "underlying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlying: Option<Vec<String>>,

    #[serde(rename = "trade_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "value_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "delivery_date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "strike")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike: Option<f64>,
}

impl TradeDetails {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> TradeDetails {
        TradeDetails {
            trading_entity: None,
            counterparty: None,
            direction: None,
            notional_currency: None,
            notional_amount: None,
            underlying: None,
            trade_date: None,
            value_date: None,
            delivery_date: None,
            strike: None,
        }
    }
}

/// Converts the TradeDetails value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TradeDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.trading_entity
                .as_ref()
                .map(|trading_entity| ["trading_entity".to_string(), trading_entity.to_string()].join(",")),
            self.counterparty
                .as_ref()
                .map(|counterparty| ["counterparty".to_string(), counterparty.to_string()].join(",")),
            self.direction.as_ref().map(|direction| ["direction".to_string(), direction.to_string()].join(",")),
            self.notional_currency
                .as_ref()
                .map(|notional_currency| ["notional_currency".to_string(), notional_currency.to_string()].join(",")),
            self.notional_amount
                .as_ref()
                .map(|notional_amount| ["notional_amount".to_string(), notional_amount.to_string()].join(",")),
            self.underlying.as_ref().map(|underlying| {
                ["underlying".to_string(), underlying.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")]
                    .join(",")
            }),
            // Skipping trade_date in query parameter serialization

            // Skipping value_date in query parameter serialization

            // Skipping delivery_date in query parameter serialization
            self.strike.as_ref().map(|strike| ["strike".to_string(), strike.to_string()].join(",")),
        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeDetails value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeDetails {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trading_entity: Vec<String>,
            pub counterparty: Vec<String>,
            pub direction: Vec<String>,
            pub notional_currency: Vec<String>,
            pub notional_amount: Vec<f64>,
            pub underlying: Vec<Vec<String>>,
            pub trade_date: Vec<chrono::DateTime<chrono::Utc>>,
            pub value_date: Vec<chrono::DateTime<chrono::Utc>>,
            pub delivery_date: Vec<chrono::DateTime<chrono::Utc>>,
            pub strike: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeDetails".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "trading_entity" => intermediate_rep
                        .trading_entity
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "counterparty" => intermediate_rep
                        .counterparty
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "direction" => intermediate_rep
                        .direction
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notional_currency" => intermediate_rep
                        .notional_currency
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "notional_amount" => intermediate_rep
                        .notional_amount
                        .push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "underlying" => {
                        return std::result::Result::Err(
                            "Parsing a container in this style is not supported in TradeDetails".to_string(),
                        )
                    }
                    #[allow(clippy::redundant_clone)]
                    "trade_date" => intermediate_rep.trade_date.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "value_date" => intermediate_rep.value_date.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "delivery_date" => intermediate_rep.delivery_date.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "strike" => intermediate_rep
                        .strike
                        .push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TradeDetails".to_string()),
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeDetails {
            trading_entity: intermediate_rep.trading_entity.into_iter().next(),
            counterparty: intermediate_rep.counterparty.into_iter().next(),
            direction: intermediate_rep.direction.into_iter().next(),
            notional_currency: intermediate_rep.notional_currency.into_iter().next(),
            notional_amount: intermediate_rep.notional_amount.into_iter().next(),
            underlying: intermediate_rep.underlying.into_iter().next(),
            trade_date: intermediate_rep.trade_date.into_iter().next(),
            value_date: intermediate_rep.value_date.into_iter().next(),
            delivery_date: intermediate_rep.delivery_date.into_iter().next(),
            strike: intermediate_rep.strike.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeDetails> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeDetails>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeDetails>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TradeDetails - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TradeDetails> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <TradeDetails as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into TradeDetails - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeDiff {
    #[serde(rename = "trade_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,

    #[serde(rename = "from_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_version: Option<i32>,

    #[serde(rename = "to_version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_version: Option<i32>,

    #[serde(rename = "differences")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differences: Option<crate::types::Object>,
}

impl TradeDiff {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> TradeDiff {
        TradeDiff { trade_id: None, from_version: None, to_version: None, differences: None }
    }
}

/// Converts the TradeDiff value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TradeDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.trade_id.as_ref().map(|trade_id| ["trade_id".to_string(), trade_id.to_string()].join(",")),
            self.from_version
                .as_ref()
                .map(|from_version| ["from_version".to_string(), from_version.to_string()].join(",")),
            self.to_version.as_ref().map(|to_version| ["to_version".to_string(), to_version.to_string()].join(",")),
            // Skipping differences in query parameter serialization
        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeDiff value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeDiff {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub trade_id: Vec<String>,
            pub from_version: Vec<i32>,
            pub to_version: Vec<i32>,
            pub differences: Vec<crate::types::Object>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeDiff".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "trade_id" => intermediate_rep
                        .trade_id
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "from_version" => intermediate_rep
                        .from_version
                        .push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to_version" => intermediate_rep
                        .to_version
                        .push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "differences" => intermediate_rep
                        .differences
                        .push(<crate::types::Object as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TradeDiff".to_string()),
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeDiff {
            trade_id: intermediate_rep.trade_id.into_iter().next(),
            from_version: intermediate_rep.from_version.into_iter().next(),
            to_version: intermediate_rep.to_version.into_iter().next(),
            differences: intermediate_rep.differences.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeDiff> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeDiff>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeDiff>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TradeDiff - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TradeDiff> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <TradeDiff as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into TradeDiff - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeEvent {
    #[serde(rename = "user_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<models::TradeDetails>,
}

impl TradeEvent {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> TradeEvent {
        TradeEvent { user_id: None, timestamp: None, state: None, details: None }
    }
}

/// Converts the TradeEvent value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TradeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            self.user_id.as_ref().map(|user_id| ["user_id".to_string(), user_id.to_string()].join(",")),
            // Skipping timestamp in query parameter serialization
            self.state.as_ref().map(|state| ["state".to_string(), state.to_string()].join(",")),
            // Skipping details in query parameter serialization
        ];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeEvent value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeEvent {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub user_id: Vec<String>,
            pub timestamp: Vec<chrono::DateTime<chrono::Utc>>,
            pub state: Vec<String>,
            pub details: Vec<models::TradeDetails>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeEvent".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "user_id" => intermediate_rep
                        .user_id
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "timestamp" => intermediate_rep.timestamp.push(
                        <chrono::DateTime<chrono::Utc> as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "state" => intermediate_rep
                        .state
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "details" => intermediate_rep
                        .details
                        .push(<models::TradeDetails as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TradeEvent".to_string()),
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeEvent {
            user_id: intermediate_rep.user_id.into_iter().next(),
            timestamp: intermediate_rep.timestamp.into_iter().next(),
            state: intermediate_rep.state.into_iter().next(),
            details: intermediate_rep.details.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeEvent> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeEvent>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeEvent>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TradeEvent - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TradeEvent> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <TradeEvent as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into TradeEvent - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TradeStatus {
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl TradeStatus {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new() -> TradeStatus {
        TradeStatus { state: None }
    }
}

/// Converts the TradeStatus value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![self.state.as_ref().map(|state| ["state".to_string(), state.to_string()].join(","))];

        write!(f, "{}", params.into_iter().flatten().collect::<Vec<_>>().join(","))
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TradeStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TradeStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub state: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TradeStatus".to_string()),
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "state" => intermediate_rep
                        .state
                        .push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TradeStatus".to_string()),
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TradeStatus { state: intermediate_rep.state.into_iter().next() })
    }
}

// Methods for converting between header::IntoHeaderValue<TradeStatus> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<TradeStatus>> for HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TradeStatus>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                "Invalid header value for TradeStatus - value: {} is invalid {}",
                hdr_value, e
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<TradeStatus> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => match <TradeStatus as std::str::FromStr>::from_str(value) {
                std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                std::result::Result::Err(err) => std::result::Result::Err(format!(
                    "Unable to convert header value '{}' into TradeStatus - {}",
                    value, err
                )),
            },
            std::result::Result::Err(e) => {
                std::result::Result::Err(format!("Unable to convert header: {:?} to string: {}", hdr_value, e))
            }
        }
    }
}
