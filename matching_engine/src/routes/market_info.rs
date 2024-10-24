use crate::ask_lib::ask::LocalAsk;
use crate::ask_lib::ask_status::AskState;
use crate::ask_lib::ask_store::LocalAskStore;
use crate::generator_lib::generator_store::GeneratorStore;
use crate::models::{
    AskInfoToSend, GeneratorInfo, GeneratorsInfoForMarket, MarketInfo, MarketInfoResponse,
    MarketStatsResponse, WelcomeResponse,
};
use actix_web::web;
use actix_web::web::Data;
use actix_web::HttpResponse;
use ethers::core::types::U256;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn market_stats(
    market_id: web::Path<String>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _generator_store: Data<Arc<RwLock<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let local_generator_store = {
        match _generator_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let market_id = market_id.into_inner();
    let market_id_u256 = U256::from_dec_str(&market_id);

    if market_id_u256.is_err() {
        return Ok(HttpResponse::BadRequest().json(MarketStatsResponse {
            market_info: "invalid market id".into(),
            generator_count: None,
            proofs_generated: None,
            proofs_pending: None,
            proofs_in_progress: None,
        }));
    }

    let market_id_u256 = market_id_u256.unwrap();

    let proofs_generated = { Some(local_ask_store.get_proof_count(&market_id_u256)) };

    let proofs_pending = {
        let asks = local_ask_store
            .get_by_ask_state_except_complete(AskState::Create)
            .result();
        if asks.is_none() {
            Some(0)
        } else {
            let asks: Vec<LocalAsk> = asks
                .unwrap()
                .into_iter()
                .filter(|a| a.market_id.eq(&market_id_u256))
                .collect();
            Some(asks.len())
        }
    };

    let proofs_in_progress = {
        let asks = local_ask_store
            .get_by_ask_state_except_complete(AskState::Assigned)
            .result();
        if asks.is_none() {
            Some(0)
        } else {
            let asks: Vec<LocalAsk> = asks
                .unwrap()
                .into_iter()
                .filter(|a| a.market_id.eq(&market_id_u256))
                .collect();
            Some(asks.len())
        }
    };

    let generator_count = {
        Some(
            local_generator_store
                .get_all_by_market_id(&market_id_u256)
                .len(),
        )
    };

    return Ok(HttpResponse::Ok().json(MarketStatsResponse {
        market_info: market_id,
        generator_count,
        proofs_generated,
        proofs_pending,
        proofs_in_progress,
    }));
}

pub async fn market_info(
    _payload: web::Json<MarketInfo>,
    _local_ask_store: Data<Arc<RwLock<LocalAskStore>>>,
    _generator_store: Data<Arc<RwLock<GeneratorStore>>>,
) -> actix_web::Result<HttpResponse> {
    let local_ask_store = {
        match _local_ask_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let local_generator_store = {
        match _generator_store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

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
        let asks = local_ask_store
            .get_by_market_id(&market_id_u256)
            .sort_by_ask_id(true)
            .result()
            .to_owned();

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
                    proving_time: local_ask_store
                        .get_proving_time(&ask.ask_id)
                        .unwrap_or_else(|| U256::zero()),
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
        let all_generators = local_generator_store.all_generators_address();

        let mut count = 0;
        let mut generators = vec![];
        for generator in all_generators {
            if let Some(generator_info) =
                local_generator_store.get_by_address_and_market(&generator, &market_id_u256)
            {
                let generator_data = { local_generator_store.get_by_address(&generator).unwrap() };
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

    return Ok(HttpResponse::Ok().json(MarketInfoResponse {
        market_info: market_id,
        asks,
        generator_info,
    }));
}
