// src/macros.rs

#[macro_export]
macro_rules! try_read_or_lock {
    ($store:expr, $local_var:ident) => {
        let $local_var = match $store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }));
            }
        };
    };
}
