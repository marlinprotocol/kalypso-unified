use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::StatusCode,
    Error,
};
use actix_web_lab::middleware::Next;

use crate::{kalypso::verify_api_key, response::response};

pub async fn api_auth(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if req.path() != "/api/generateApiKey" {
        let api_key = req.headers().get("api-key");
        if api_key.is_none() {
            let (request, _pl) = req.into_parts();
            let resp = response(
                "api-key not provided in the headers",
                StatusCode::BAD_REQUEST,
                None,
            )
            .map_into_right_body();
            return Ok(ServiceResponse::new(request, resp));
        } else {
            let api_key_string = api_key.unwrap().to_str().unwrap();
            let authentication_status_call = match verify_api_key(api_key_string).await {
                Ok(data) => data,
                Err(e) => {
                    log::error!("{}", e);
                    let (request, _pl) = req.into_parts();
                    let resp = response(
                        "There was an issue in verifying the api-key provided",
                        StatusCode::INTERNAL_SERVER_ERROR,
                        None,
                    )
                    .map_into_right_body();
                    return Ok(ServiceResponse::new(request, resp));
                }
            };
            if !authentication_status_call.status {
                let (request, _pl) = req.into_parts();
                let resp = response(
                    &authentication_status_call.message,
                    StatusCode::UNAUTHORIZED,
                    None,
                )
                .map_into_right_body();
                return Ok(ServiceResponse::new(request, resp));
            }
        }
    }
    let res = next.call(req).await?;
    Ok(res.map_into_left_body())
}
