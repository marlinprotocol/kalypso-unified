use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::X509Builder;
use openssl::x509::X509Name;
use rustls::{Certificate, PrivateKey};

pub fn generate_self_signed_cert() -> (Vec<Certificate>, PrivateKey) {
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

    // Set certificate properties
    builder.set_subject_name(&name).unwrap();
    builder.set_issuer_name(&name).unwrap();
    builder.set_pubkey(&pkey).unwrap();

    // Set certificate validity (from now to 365 days from now)
    let now = openssl::asn1::Asn1Time::days_from_now(0).unwrap(); // Valid from now
    let expire = openssl::asn1::Asn1Time::days_from_now(365).unwrap(); // Valid for 1 year
    builder.set_not_before(&now).unwrap();
    builder.set_not_after(&expire).unwrap();

    // Sign the certificate with SHA-256
    builder.sign(&pkey, MessageDigest::sha256()).unwrap();

    // Convert to PEM format
    let cert = builder.build();
    let cert_pem = cert.to_pem().unwrap();
    let key_pem = pkey.private_key_to_pem_pkcs8().unwrap();

    // Convert PEM to rustls types
    let cert_chain = vec![Certificate(cert_pem)];
    let private_key = PrivateKey(key_pem);

    (cert_chain, private_key)
}
