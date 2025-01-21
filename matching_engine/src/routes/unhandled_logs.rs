use crate::models::WelcomeResponse;
use actix_web::web::Data;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use ethers::types::Log;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Result {
    result: Vec<String>,
}

#[utoipa::path(
    get,
    path = "/stats/unhandled_logs",
    responses(
        (status = 200, description = "Return vector of unhandled of logs"),
    ),
    tag = "Manage"
)]
pub async fn get_unhandled_logs(
    _shared_unhandled_logs: Data<Arc<RwLock<Vec<Log>>>>,
) -> actix_web::Result<HttpResponse> {
    let unhandled_logs = {
        match _shared_unhandled_logs.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }))
            }
        }
    };

    let mut to_return = vec![];
    for log in unhandled_logs.iter() {
        to_return.push(hex::encode(log.transaction_hash.unwrap()));
    }

    return Ok(HttpResponse::Ok().json(Result { result: to_return }));
}
