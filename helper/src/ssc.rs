use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::X509Builder;
use openssl::x509::X509Name;

pub struct CertInfo {
    pub cert_pem: Vec<u8>,
    pub key_der: Vec<u8>, // Use DER format for Rustls
}

pub fn generate_self_signed_cert() -> CertInfo {
    // Generate RSA key
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    // Create X509 builder and build self-signed cert
    let mut builder = X509Builder::new().unwrap();

    // Set certificate name
    let mut name_builder = X509Name::builder().unwrap();
    name_builder
        .append_entry_by_text("CN", "localhost")
        .unwrap();
    let name = name_builder.build();

    builder.set_subject_name(&name).unwrap();
    builder.set_issuer_name(&name).unwrap();
    builder.set_pubkey(&pkey).unwrap();

    // Set certificate validity
    let now = openssl::asn1::Asn1Time::days_from_now(0).unwrap(); // Valid from now
    let expire = openssl::asn1::Asn1Time::days_from_now(365).unwrap(); // Valid for 1 year
    builder.set_not_before(&now).unwrap();
    builder.set_not_after(&expire).unwrap();

    // Sign the certificate with SHA-256
    builder.sign(&pkey, MessageDigest::sha256()).unwrap();

    // Convert certificate and key to correct formats
    let cert = builder.build();
    let cert_pem = cert.to_pem().unwrap(); // Certificate in PEM format
    let key_der = pkey.private_key_to_der().unwrap(); // Private key in DER format

    CertInfo { cert_pem, key_der }
}

use rustls::{Certificate, PrivateKey, ServerConfig};

pub fn create_rustls_server_config(
    cert_pem: Vec<u8>,
    key_pem: Vec<u8>,
) -> Result<ServerConfig, rustls::Error> {
    let cert_chain = vec![Certificate(cert_pem)];
    let private_key = PrivateKey(key_pem);

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
}

pub fn create_random_rustls_server_config() -> Result<ServerConfig, rustls::Error> {
    let cert_info = generate_self_signed_cert();
    let cert_chain = vec![Certificate(cert_info.cert_pem)];
    let private_key = PrivateKey(cert_info.key_der);

    ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
}
