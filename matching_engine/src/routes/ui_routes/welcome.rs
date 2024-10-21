use crate::models::WelcomeResponse;
use actix_web::HttpResponse;

pub async fn welcome() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(WelcomeResponse {
        status: "welcome to ui routes!.".into(),
    }))
}
