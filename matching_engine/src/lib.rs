pub mod ask;
pub mod generator;
pub mod middlewares;
pub mod models;
pub mod utility;

use models::{GetAskStatus, MarketInfo};
use reqwest::StatusCode;
use service_check_helper::{Request, RequestType};

pub fn get_welcome_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/welcome".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks server reach".into(),
    }
}

pub fn get_latest_block_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getLatestBlock".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks upto which block ME has reached".into(),
    }
}

pub fn get_key_balance_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getKeyBalance".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks balance of gas key".into(),
    }
}

pub fn get_status_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getStatus".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code: StatusCode::OK,
        info: "Checks ME overal status".into(),
    }
}

pub fn get_single_ask_status_request<R>(
    input_payload: Option<GetAskStatus>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<GetAskStatus, R> {
    Request {
        request_type: RequestType::POST(
            input_payload.unwrap_or_else(|| GetAskStatus { ask_id: "1".into() }),
        ),
        service_endpoint: "/getAskStatus".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}

pub fn get_single_market_info<R>(
    input_payload: Option<MarketInfo>,
    expected_status_code: StatusCode,
    info: String,
) -> Request<MarketInfo, R> {
    Request {
        request_type: RequestType::POST(input_payload.unwrap_or_else(|| MarketInfo {
            market_id: "3".into(),
        })),
        service_endpoint: "/marketInfo".into(),
        _marker: std::marker::PhantomData::<R>,
        expected_status_code,
        info,
    }
}
