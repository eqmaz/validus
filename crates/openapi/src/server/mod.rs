use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::models;

use crate::{Api,
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

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: Api + 'static,
{
    // build our application with a route
    Router::new()
        .route("/hello",
            get(hello::<I, A>)
        )
        .route("/trade",
            get(list_trades::<I, A>).post(create_trade::<I, A>)
        )
        .route("/trade/:id",
            delete(cancel_trade::<I, A>).get(get_trade_status::<I, A>)
        )
        .route("/trade/:id/approve",
            post(approve_trade::<I, A>)
        )
        .route("/trade/:id/book",
            post(book_trade::<I, A>)
        )
        .route("/trade/:id/details",
            get(get_trade_details::<I, A>).put(update_trade::<I, A>)
        )
        .route("/trade/:id/diff",
            get(trade_diff::<I, A>)
        )
        .route("/trade/:id/history",
            get(get_trade_history::<I, A>)
        )
        .route("/trade/:id/send",
            post(send_trade::<I, A>)
        )
        .route("/trade/:id/submit",
            post(submit_trade::<I, A>)
        )
        .with_state(api_impl)
}


#[tracing::instrument(skip_all)]
fn approve_trade_validation(
  path_params: models::ApproveTradePathParams,
) -> std::result::Result<(
  models::ApproveTradePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// ApproveTrade - POST /trade/{id}/approve
#[tracing::instrument(skip_all)]
async fn approve_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::ApproveTradePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    approve_trade_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().approve_trade(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                ApproveTradeResponse::Status204_TradeApproved
                                                => {

                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn book_trade_validation(
  path_params: models::BookTradePathParams,
) -> std::result::Result<(
  models::BookTradePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// BookTrade - POST /trade/{id}/book
#[tracing::instrument(skip_all)]
async fn book_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::BookTradePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    book_trade_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().book_trade(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                BookTradeResponse::Status204_TradeBooked
                                                => {

                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn cancel_trade_validation(
  path_params: models::CancelTradePathParams,
) -> std::result::Result<(
  models::CancelTradePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// CancelTrade - DELETE /trade/{id}
#[tracing::instrument(skip_all)]
async fn cancel_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::CancelTradePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    cancel_trade_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().cancel_trade(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                CancelTradeResponse::Status204_TradeCancelled
                                                => {

                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct CreateTradeBodyValidator<'a> {
            #[validate]
          body: &'a models::TradeCreateRequest,
    }


#[tracing::instrument(skip_all)]
fn create_trade_validation(
        body: models::TradeCreateRequest,
) -> std::result::Result<(
        models::TradeCreateRequest,
), ValidationErrors>
{
              let b = CreateTradeBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}

/// CreateTrade - POST /trade
#[tracing::instrument(skip_all)]
async fn create_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::TradeCreateRequest>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    create_trade_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().create_trade(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                CreateTradeResponse::Status200_TradeCreated
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_trade_details_validation(
  path_params: models::GetTradeDetailsPathParams,
) -> std::result::Result<(
  models::GetTradeDetailsPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// GetTradeDetails - GET /trade/{id}/details
#[tracing::instrument(skip_all)]
async fn get_trade_details<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTradeDetailsPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    get_trade_details_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().get_trade_details(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetTradeDetailsResponse::Status200_FullTradeDetails
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_trade_history_validation(
  path_params: models::GetTradeHistoryPathParams,
) -> std::result::Result<(
  models::GetTradeHistoryPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// GetTradeHistory - GET /trade/{id}/history
#[tracing::instrument(skip_all)]
async fn get_trade_history<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTradeHistoryPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    get_trade_history_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().get_trade_history(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetTradeHistoryResponse::Status200_FullTradeState
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn get_trade_status_validation(
  path_params: models::GetTradeStatusPathParams,
) -> std::result::Result<(
  models::GetTradeStatusPathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// GetTradeStatus - GET /trade/{id}
#[tracing::instrument(skip_all)]
async fn get_trade_status<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::GetTradeStatusPathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    get_trade_status_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().get_trade_status(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                GetTradeStatusResponse::Status200_CurrentTradeStatus
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn hello_validation(
) -> std::result::Result<(
), ValidationErrors>
{

Ok((
))
}

/// Hello - GET /hello
#[tracing::instrument(skip_all)]
async fn hello<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    hello_validation(
    )
  ).await.unwrap();

  let Ok((
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().hello(
      method,
      host,
      cookies,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                HelloResponse::Status200_ReturnsAWelcomeMessage
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn list_trades_validation(
  query_params: models::ListTradesQueryParams,
) -> std::result::Result<(
  models::ListTradesQueryParams,
), ValidationErrors>
{
  query_params.validate()?;

Ok((
  query_params,
))
}

/// ListTrades - GET /trade
#[tracing::instrument(skip_all)]
async fn list_trades<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Query(query_params): Query<models::ListTradesQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    list_trades_validation(
        query_params,
    )
  ).await.unwrap();

  let Ok((
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().list_trades(
      method,
      host,
      cookies,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                ListTradesResponse::Status200_ListOfTradeIDs
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn send_trade_validation(
  path_params: models::SendTradePathParams,
) -> std::result::Result<(
  models::SendTradePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// SendTrade - POST /trade/{id}/send
#[tracing::instrument(skip_all)]
async fn send_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::SendTradePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    send_trade_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().send_trade(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                SendTradeResponse::Status204_TradeSent
                                                => {

                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn submit_trade_validation(
  path_params: models::SubmitTradePathParams,
) -> std::result::Result<(
  models::SubmitTradePathParams,
), ValidationErrors>
{
  path_params.validate()?;

Ok((
  path_params,
))
}

/// SubmitTrade - POST /trade/{id}/submit
#[tracing::instrument(skip_all)]
async fn submit_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::SubmitTradePathParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    submit_trade_validation(
        path_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().submit_trade(
      method,
      host,
      cookies,
        path_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                SubmitTradeResponse::Status204_TradeSubmitted
                                                => {

                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}


#[tracing::instrument(skip_all)]
fn trade_diff_validation(
  path_params: models::TradeDiffPathParams,
  query_params: models::TradeDiffQueryParams,
) -> std::result::Result<(
  models::TradeDiffPathParams,
  models::TradeDiffQueryParams,
), ValidationErrors>
{
  path_params.validate()?;
  query_params.validate()?;

Ok((
  path_params,
  query_params,
))
}

/// TradeDiff - GET /trade/{id}/diff
#[tracing::instrument(skip_all)]
async fn trade_diff<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::TradeDiffPathParams>,
  Query(query_params): Query<models::TradeDiffQueryParams>,
 State(api_impl): State<I>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    trade_diff_validation(
        path_params,
        query_params,
    )
  ).await.unwrap();

  let Ok((
    path_params,
    query_params,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().trade_diff(
      method,
      host,
      cookies,
        path_params,
        query_params,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                TradeDiffResponse::Status200_FieldDifferencesBetweenTwoVersions
                                                    (body)
                                                => {

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct UpdateTradeBodyValidator<'a> {
            #[validate]
          body: &'a models::TradeDetails,
    }


#[tracing::instrument(skip_all)]
fn update_trade_validation(
  path_params: models::UpdateTradePathParams,
        body: models::TradeDetails,
) -> std::result::Result<(
  models::UpdateTradePathParams,
        models::TradeDetails,
), ValidationErrors>
{
  path_params.validate()?;
              let b = UpdateTradeBodyValidator { body: &body };
              b.validate()?;

Ok((
  path_params,
    body,
))
}

/// UpdateTrade - PUT /trade/{id}/details
#[tracing::instrument(skip_all)]
async fn update_trade<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
  Path(path_params): Path<models::UpdateTradePathParams>,
 State(api_impl): State<I>,
          Json(body): Json<models::TradeDetails>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    update_trade_validation(
        path_params,
          body,
    )
  ).await.unwrap();

  let Ok((
    path_params,
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().update_trade(
      method,
      host,
      cookies,
        path_params,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                UpdateTradeResponse::Status204_TradeUpdated
                                                => {

                                                  let mut response = response.status(204);
                                                  response.body(Body::empty())
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

