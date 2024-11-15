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

#[macro_export]
macro_rules! try_read_and_get_if_valid {
    ($store:expr, $cache_var:ident, $duration:expr) => {
        let $cache_var = match $store.try_read() {
            Ok(data) => data,
            _ => {
                return Ok(HttpResponse::Locked().json(WelcomeResponse {
                    status: "Resource Busy".into(),
                }));
            }
        };

        if let Some(response) = $cache_var.get_if_valid($duration) {
            return Ok(HttpResponse::Ok().json(response));
        }
    };
}
