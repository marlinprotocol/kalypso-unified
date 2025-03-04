pub mod ask_lib;
pub mod costs;
pub mod counters;
pub mod dump;
pub mod encrypted_dump;
pub mod generator_lib;
pub mod in_memory_matching_engine;
pub mod in_memory_stores;
pub mod latest_block_store;
pub mod market_metadata;
pub mod models;
pub mod utility;

mod jobs;
mod log_processor;
mod routes;

#[macro_use]
mod macros;

use models::{GetAskStatus, MarketInfo};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchingEngineConfig {
    pub rpc_url: String,
    pub chain_id: String,
    pub matching_engine_key: String,
    pub relayer_private_key: String,
    pub proof_market_place: String,
    pub generator_registry: String,
    pub entity_registry: String,
    pub symbiotic_staking: String,
    pub native_staking: String,

    #[serde(alias = "stake_manager")]
    pub staking_manager: String,
    pub start_block: String,
}
