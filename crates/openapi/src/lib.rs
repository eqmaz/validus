#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0.0";

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ApproveTradeResponse {
    /// Trade approved
    Status204_TradeApproved
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum BookTradeResponse {
    /// Trade booked
    Status204_TradeBooked
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CancelTradeResponse {
    /// Trade cancelled
    Status204_TradeCancelled
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateTradeResponse {
    /// Trade created
    Status200_TradeCreated
    (models::TradeCreateResponse)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTradeDetailsResponse {
    /// Full trade details
    Status200_FullTradeDetails
    (models::TradeDetails)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTradeHistoryResponse {
    /// Full trade state/event history
    Status200_FullTradeState
    (Vec<models::TradeEvent>)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetTradeStatusResponse {
    /// Current trade status
    Status200_CurrentTradeStatus
    (models::TradeStatus)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum HelloResponse {
    /// Returns a welcome message
    Status200_ReturnsAWelcomeMessage
    (models::HelloResponse)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum ListTradesResponse {
    /// List of trade IDs
    Status200_ListOfTradeIDs
    (Vec<String>)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SendTradeResponse {
    /// Trade sent
    Status204_TradeSent
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum SubmitTradeResponse {
    /// Trade submitted
    Status204_TradeSubmitted
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TradeDiffResponse {
    /// Field differences between two versions
    Status200_FieldDifferencesBetweenTwoVersions
    (models::TradeDiff)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateTradeResponse {
    /// Trade updated
    Status204_TradeUpdated
}


/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {

                /// Approve a trade.
                ///
                /// ApproveTrade - POST /trade/{id}/approve
                async fn approve_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::ApproveTradePathParams,
                ) -> Result<ApproveTradeResponse, String>;


                /// Mark a trade as executed.
                ///
                /// BookTrade - POST /trade/{id}/book
                async fn book_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::BookTradePathParams,
                ) -> Result<BookTradeResponse, String>;


                /// Cancel a trade.
                ///
                /// CancelTrade - DELETE /trade/{id}
                async fn cancel_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::CancelTradePathParams,
                ) -> Result<CancelTradeResponse, String>;


                /// Create a new trade.
                ///
                /// CreateTrade - POST /trade
                async fn create_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: models::TradeCreateRequest,
                ) -> Result<CreateTradeResponse, String>;


                /// Get trade details.
                ///
                /// GetTradeDetails - GET /trade/{id}/details
                async fn get_trade_details(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::GetTradeDetailsPathParams,
                ) -> Result<GetTradeDetailsResponse, String>;


                /// Get trade history.
                ///
                /// GetTradeHistory - GET /trade/{id}/history
                async fn get_trade_history(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::GetTradeHistoryPathParams,
                ) -> Result<GetTradeHistoryResponse, String>;


                /// Get trade status.
                ///
                /// GetTradeStatus - GET /trade/{id}
                async fn get_trade_status(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::GetTradeStatusPathParams,
                ) -> Result<GetTradeStatusResponse, String>;


                /// Hello World endpoint.
                ///
                /// Hello - GET /hello
                async fn hello(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                ) -> Result<HelloResponse, String>;


                /// List trade IDs.
                ///
                /// ListTrades - GET /trade
                async fn list_trades(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  query_params: models::ListTradesQueryParams,
                ) -> Result<ListTradesResponse, String>;


                /// Send trade to counterparty.
                ///
                /// SendTrade - POST /trade/{id}/send
                async fn send_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::SendTradePathParams,
                ) -> Result<SendTradeResponse, String>;


                /// Submit a draft trade for approval.
                ///
                /// SubmitTrade - POST /trade/{id}/submit
                async fn submit_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::SubmitTradePathParams,
                ) -> Result<SubmitTradeResponse, String>;


                /// Compare two trade versions.
                ///
                /// TradeDiff - GET /trade/{id}/diff
                async fn trade_diff(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::TradeDiffPathParams,
                  query_params: models::TradeDiffQueryParams,
                ) -> Result<TradeDiffResponse, String>;


                /// Update trade details.
                ///
                /// UpdateTrade - PUT /trade/{id}/details
                async fn update_trade(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                  path_params: models::UpdateTradePathParams,
                        body: models::TradeDetails,
                ) -> Result<UpdateTradeResponse, String>;

}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
