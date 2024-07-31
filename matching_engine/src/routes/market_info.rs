use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::{Address, U256};
use matching_engine::ask::*;
use matching_engine::generator::{GeneratorState, GeneratorStore};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct MarketInfo {
    market_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd)]
pub struct AskInfoToSend {
    pub ask_id: U256,
    pub market_id: U256,
    pub reward: U256,
    pub expiry: U256,
    pub proving_time: U256,
    pub deadline: U256,
    pub has_private_inputs: bool,
    pub state: Option<AskState>,
    pub generator: Option<Address>,
}

#[derive(Serialize)]
pub struct MarketInfoResponse {
    market_info: String,
    asks: Option<Vec<AskInfoToSend>>,
    generator_info: Option<GeneratorsInfoForMarket>,
}

#[derive(Serialize)]
pub struct GeneratorInfo {
    generator_address: Address,
    stake_locked: U256,
    total_stake: U256,
    compute_consumed: U256,
    declared_compute: U256,
    compute_required_per_request: U256,
    proof_generation_cost: U256,
    proposed_time: U256,
    active_requests: U256,
    proofs_submitted: U256,
    state: Option<GeneratorState>,
}

#[derive(Serialize)]
pub struct GeneratorsInfoForMarket {
    count: i32,
    generators: Vec<GeneratorInfo>,
}

pub async fn market_info(
    _payload: web::Json<MarketInfo>,
    _local_ask_store: Data<Arc<Mutex<LocalAskStore>>>,
    _generator_store: Data<Arc<Mutex<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    let market_id: String = _payload.market_id.clone();
    let market_id_u256 = U256::from_dec_str(&market_id);

    if market_id_u256.is_err() {
        return Ok(HttpResponse::BadRequest().json(MarketInfoResponse {
            market_info: "invalid market id".into(),
            asks: None,
            generator_info: None,
        }));
    }

    let market_id_u256 = market_id_u256.unwrap();

    let asks = {
        let local_ask_store = { _local_ask_store.lock().await };
        let asks = local_ask_store
            .get_by_market_id(&market_id_u256)
            .sort_by_ask_id()
            .result();

        if asks.is_none() {
            None
        } else {
            let asks = asks.unwrap();
            let asks: Vec<AskInfoToSend> = asks
                .iter()
                .map(|ask| AskInfoToSend {
                    ask_id: ask.ask_id,
                    market_id: ask.market_id,
                    reward: ask.reward,
                    expiry: ask.expiry,
                    proving_time: ask.proving_time,
                    deadline: ask.deadline,
                    has_private_inputs: ask.has_private_inputs,
                    state: ask.state,
                    generator: ask.generator,
                })
                .collect();
            Some(asks)
        }
    };

    let generator_info = {
        let generator_store = { _generator_store.lock().await };
        let all_generators = generator_store.clone().all_generators_address();

        let mut count = 0;
        let mut generators = vec![];
        for generator in all_generators {
            if let Some(generator_info) =
                generator_store.get_by_address_and_market(&generator, &market_id_u256)
            {
                let generator_data = { generator_store.get_by_address(&generator).unwrap() };
                count += 1;
                generators.push(GeneratorInfo {
                    generator_address: generator,
                    stake_locked: generator_data.stake_locked,
                    total_stake: generator_data.total_stake,
                    compute_consumed: generator_data.compute_consumed,
                    declared_compute: generator_data.declared_compute,
                    compute_required_per_request: generator_info.compute_required_per_request,
                    proof_generation_cost: generator_info.proof_generation_cost,
                    proposed_time: generator_info.proposed_time,
                    active_requests: generator_info.active_requests,
                    proofs_submitted: generator_info.proofs_submitted,
                    state: generator_info.state,
                })
            }
        }
        Some(GeneratorsInfoForMarket { count, generators })
    };

    Ok(HttpResponse::Ok().json(MarketInfoResponse {
        market_info: market_id,
        asks,
        generator_info,
    }))
}
