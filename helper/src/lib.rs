pub mod attestation_helpers;
pub mod common_handlers;
pub mod custom_logger;
pub mod image_id_helpers;
pub mod middlewares;
pub mod pcr_helpers;
pub mod prom_client;
pub mod response;
pub mod sch_request;
pub mod sch_response;
pub mod secret_inputs_helpers;
pub mod ssc;

// added this because oyster-sdk-rs import is not directly working
#[allow(unused)]
// #[deprecated(note = "Use `oyster_monorepo` directly for verification")]
mod oyster;

#[macro_export]
macro_rules! try_read_contract_error {
    ($error:expr, $error_type:path, $contract_label:expr) => {{
        if let Some(contract_error) = $error.decode_contract_revert::<$error_type>() {
            eprintln!("{:?} Error: {:?}", $contract_label, contract_error);
        }
    }};
}

#[macro_export]
macro_rules! try_read_contract_error_log {
    ($error:expr, $error_type:path, $contract_label:expr) => {{
        if let Some(contract_error) = $error.decode_contract_revert::<$error_type>() {
            log::error!("{:?} Error: {:?}", $contract_label, contract_error);
        }
    }};
}

#[macro_export]
macro_rules! send_with_optional_gas {
    ($builder:expr) => {{
        // Initialize the transaction builder as mutable
        let mut __tx_builder = $builder;

        // Conditionally set the gas limit based on the feature flag
        if cfg!(feature = "force_transactions") {
            __tx_builder = __tx_builder.gas(1_000_000);
        }

        // Send and confirm the transaction, handling errors
        CommonDeps::send_and_confirm(__tx_builder.send()).await
    }};
}
