use crate::ask::*;
use crate::generator::GeneratorStore;
use crate::models::{
    AskInfoToSend, GeneratorInfo, GeneratorsInfoForMarket, MarketInfo, MarketInfoResponse,
};
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use std::sync::Arc;
use tokio::sync::Mutex;

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
