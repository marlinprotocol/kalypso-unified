use async_trait::async_trait;
use matching_engine_helpers::generator_lib::generator_store::GeneratorMeta;
use matching_engine_helpers::utility::TokenAmount;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Once;

use crate::common_deps::CommonDeps;

use super::Operation;

pub struct CheckOtherOperators;
static OPERATOR_INIT: Once = Once::new();

#[async_trait]
impl Operation for CheckOtherOperators {
    async fn execute(&self, config: HashMap<String, String>) -> Result<(), String> {
        let info = CommonDeps::read_all_operator_query_info(&config)?;
        let info = get_single_market_info(info.indexer_url, info.market_id.to_string())
            .await
            .map_err(|e| format!("Failed Fetching Operator Info From Indexer: {}", e))?;

        println!("\n");

        for operator in info.registered_generator_list {
            OPERATOR_INIT.call_once(|| {
                // Print the multi-line header
                println!(
                    "{:<42} | {:<30} | {:<75} | {:>20}",
                    "Operator", "Proof Generation Cost", "Total Delegations", "Time"
                );

                println!(
                    "{:-<42}-+-{:-<30}-+-{:-<42}-+-{:-<30}-+-{:-<20}",
                    "", "", "", "", ""
                );
                println!(
                    "{:<42} | {:<30} | {:<42} | {:<30} | {:>20}",
                    "", "", "Token", "Amount", ""
                );
                println!(
                    "{:-<42}-+-{:-<30}-+-{:-<42}-+-{:-<30}-+-{:-<20}",
                    "", "", "", "", ""
                );
            });

            if operator.delegations.is_empty() {
                println!(
                    "{:<42} | {:<30} | {:<42} | {:<30} | {:>20}",
                    operator
                        .details
                        .display_name
                        .clone()
                        .unwrap_or(operator.address.clone()),
                    operator.cost.amount.to_string(),
                    "-",
                    "-",
                    operator.time.clone()
                );
            } else {
                // Print operator data with delegations as sub-columns
                for delegation in &operator.delegations {
                    println!(
                        "{:<42} | {:<30} | {:<42} | {:<30} | {:>20}",
                        if delegation.token == operator.delegations[0].token {
                            operator
                                .details
                                .display_name
                                .clone()
                                .unwrap_or(operator.address.clone())
                        } else {
                            "".to_string()
                        },
                        if delegation.token == operator.delegations[0].token {
                            operator.cost.amount.to_string()
                        } else {
                            "".to_string()
                        },
                        delegation.token,
                        delegation.amount,
                        if delegation.token == operator.delegations[0].token {
                            operator.time.clone()
                        } else {
                            "".to_string()
                        }
                    );
                }
            }
            println!(
                "{:-<42}-+-{:-<30}-+-{:-<42}-+-{:-<30}-+-{:-<20}",
                "", "", "", "", ""
            );
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RegisteredGenerator {
    details: GeneratorMeta,
    address: String,
    delegations: Vec<TokenAmount>,
    time: String,
    cost: TokenAmount,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SingleMarketResponse {
    registered_generator_list: Vec<RegisteredGenerator>,
}

async fn get_single_market_info(
    indexer_url: String,
    market_id: String,
) -> Result<SingleMarketResponse, Box<dyn Error>> {
    let url = format!("{}/ui/market/{}", indexer_url, market_id);
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let market_query_result: SingleMarketResponse = response.json().await?;
        return Ok(market_query_result);
    } else {
        log::error!(
            "Failed to fetch market/{}: HTTP {}",
            market_id,
            response.status()
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Failed to fetch market/{}: HTTP {}",
                market_id,
                response.status()
            ),
        )));
    }
}
