use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct JsonResponse {
    status: String,
    message: String,
    data: Option<Value>,
}

struct ResponseOptions {
    data: Option<Value>,
    message: String,
    status_code: StatusCode,
}

struct ResponseHandler {
    options: ResponseOptions,
}

impl ResponseHandler {
    fn new(options: ResponseOptions) -> Self {
        ResponseHandler { options }
    }

    fn create_json_response(self) -> JsonResponse {
        let status = if self.options.status_code == StatusCode::OK {
            "success".to_string()
        } else {
            "error".to_string()
        };

        JsonResponse {
            status,
            message: self.options.message,
            data: self.options.data,
        }
    }

    fn create_http_response(self) -> HttpResponse {
        let status_code = self.options.status_code;
        let json_resp = self.create_json_response();
        HttpResponse::build(status_code).json(json_resp)
    }
}

//Generate response
pub fn response(
    message: &str,
    status_code: StatusCode,
    data: Option<serde_json::Value>,
) -> HttpResponse {
    let options = ResponseOptions {
        data,
        message: message.to_string(),
        status_code,
    };

    let response_handler = ResponseHandler::new(options);
    response_handler.create_http_response()
}
