#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use async_trait::async_trait;
use axum::{extract::Host, http::Method, Json};
use axum_extra::extract::CookieJar;
use openapi::{
    Api,
    ApproveTradeResponse,
    BookTradeResponse,
    CancelTradeResponse,
    CreateTradeResponse,
    GetTradeDetailsResponse,
    GetTradeHistoryResponse,
    GetTradeStatusResponse,
    HelloResponse,
    ListTradesResponse,
    SendTradeResponse,
    SubmitTradeResponse,
    TradeDiffResponse,
    UpdateTradeResponse
};
use openapi::models::{
    ApproveTradePathParams,
    BookTradePathParams,
    CancelTradePathParams,
    GetTradeDetailsPathParams,
    GetTradeHistoryPathParams,
    GetTradeStatusPathParams,
    ListTradesQueryParams,
    SendTradePathParams,
    SubmitTradePathParams,
    TradeCreateRequest,
    TradeDetails,
    TradeDiffPathParams,
    TradeDiffQueryParams,
    UpdateTradePathParams
};
use crate::service::trading_service;
use crate::service::mapper;

#[derive(Default, Clone)]
pub struct RestApiImpl;

#[async_trait]
impl Api for RestApiImpl {

    /// Create a new trade ino draft status
    async fn create_trade(
        &self,
        method: Method,
        host: Host,
        cookies: CookieJar,
        raw_body: TradeCreateRequest, // required by trait
    ) -> Result<CreateTradeResponse, String> {
        let user_id = raw_body.user_id.clone().ok_or("Missing user_id")?;
        let details_api = raw_body.details.clone().ok_or("Missing trade details")?;

        let trade_details = mapper::to_trade_details(&details_api)
            .map_err(|e| format!("Invalid trade details: {e:?}"))?;

        let trade_id = trading_service::create_trade(&user_id, trade_details)
            .map_err(|e| format!("Trade creation failed: {e:?}"))?;

        Ok(CreateTradeResponse::Status200_TradeCreated(
            openapi::models::TradeCreateResponse {
                trade_id: Some(trade_id),
            },
        ))
    }


    async fn get_trade_history(&self, method: Method, host: Host, cookies: CookieJar, path_params: GetTradeHistoryPathParams) -> Result<GetTradeHistoryResponse, String> {
        let trade_id = path_params.id.clone();

        // convert the trade_id from String to u64
        let trade_id = trade_id.parse::<u64>()
            .map_err(|e| format!("Invalid trade ID: {e:?}"))?;

        // history_date is Vector of TradeEventSnapshot
        let history_data = trading_service::trade_history(trade_id).map_err(|e| e.to_string())?;

        // We need to convert TradeEventSnapshot to JSON
        let history_json = mapper::to_history_response(&history_data).map_err(|e| e.to_string())?;


        Ok(GetTradeHistoryResponse::Status200_FullTradeState(history_json))
    }

    async fn approve_trade(&self, method: Method, host: Host, cookies: CookieJar, path_params: ApproveTradePathParams) -> Result<ApproveTradeResponse, String> {
        todo!()
    }

    async fn book_trade(&self, method: Method, host: Host, cookies: CookieJar, path_params: BookTradePathParams) -> Result<BookTradeResponse, String> {
        todo!()
    }

    async fn cancel_trade(&self, method: Method, host: Host, cookies: CookieJar, path_params: CancelTradePathParams) -> Result<CancelTradeResponse, String> {
        todo!()
    }



    async fn get_trade_details(&self, method: Method, host: Host, cookies: CookieJar, path_params: GetTradeDetailsPathParams) -> Result<GetTradeDetailsResponse, String> {
        todo!()
    }


    async fn get_trade_status(&self, method: Method, host: Host, cookies: CookieJar, path_params: GetTradeStatusPathParams) -> Result<GetTradeStatusResponse, String> {
        todo!()
    }

    async fn hello(&self, _method: Method, _host: Host, _cookies: CookieJar) -> Result<HelloResponse, String> {
        Ok(HelloResponse::Status200_ReturnsAWelcomeMessage(
            openapi::models::HelloResponse {
                message: Some("Hello World".to_string()),
            },
        ))
    }

    async fn list_trades(&self, method: Method, host: Host, cookies: CookieJar, query_params: ListTradesQueryParams) -> Result<ListTradesResponse, String> {
        todo!()
    }

    async fn send_trade(&self, method: Method, host: Host, cookies: CookieJar, path_params: SendTradePathParams) -> Result<SendTradeResponse, String> {
        todo!()
    }

    async fn submit_trade(&self, method: Method, host: Host, cookies: CookieJar, path_params: SubmitTradePathParams) -> Result<SubmitTradeResponse, String> {
        todo!()
    }

    async fn trade_diff(&self, method: Method, host: Host, cookies: CookieJar, path_params: TradeDiffPathParams, query_params: TradeDiffQueryParams) -> Result<TradeDiffResponse, String> {
        todo!()
    }

    async fn update_trade(&self, method: Method, host: Host, cookies: CookieJar, path_params: UpdateTradePathParams, body: TradeDetails) -> Result<UpdateTradeResponse, String> {
        todo!()
    }
}
