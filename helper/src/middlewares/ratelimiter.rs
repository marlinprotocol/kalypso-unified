use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInput, SimpleInputFunctionBuilder, SimpleOutput},
    RateLimiter,
};
use actix_web::dev::ServiceRequest;
use std::future::Ready;
use std::time::Duration;

pub fn get_rate_limiter() -> RateLimiter<
    InMemoryBackend,
    SimpleOutput,
    impl Fn(&ServiceRequest) -> Ready<Result<SimpleInput, actix_web::Error>> + 'static,
> {
    let backend = InMemoryBackend::builder().build();
    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(1), 5)
        .real_ip_key()
        .build();

    RateLimiter::builder(backend.clone(), input)
        .add_headers()
        .build()
}
