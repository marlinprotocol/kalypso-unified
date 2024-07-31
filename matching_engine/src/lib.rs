pub mod ask;
pub mod generator;
pub mod middlewares;
pub mod models;
pub mod utility;

use service_check_helper::{Request, RequestType};

pub fn get_welcome_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/welcome".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn get_latest_block_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getLatestBlock".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}

pub fn get_status_request<R>() -> Request<(), R> {
    Request {
        request_type: RequestType::GET,
        service_endpoint: "/getStatus".into(),
        _marker: std::marker::PhantomData::<R>,
    }
}
