use std::collections::BTreeMap;

use aws_nitro_enclaves_cose::{crypto::Openssl, CoseSign1};
use chrono::Utc;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::Uri;
use hyper_util::client::legacy::{Client, Error};
use hyper_util::rt::TokioExecutor;
use openssl::asn1::Asn1Time;
use openssl::x509::{X509VerifyResult, X509};
use serde_cbor::{self, value, value::Value};

#[derive(Debug)]
pub struct AttestationDecoded {
    pub pcrs: [[u8; 48]; 3],
    pub timestamp: usize,
    pub public_key: Vec<u8>,
}

#[derive(thiserror::Error, Debug)]
pub enum AttestationError {
    #[error("failed to parse: {0}")]
    ParseFailed(String),
    #[error("failed to verify attestation: {0}")]
    VerifyFailed(String),
    #[error("http client error")]
    HttpClientError(#[from] Error),
    #[error("http body error")]
    HttpBodyError(#[from] hyper::Error),
}

fn get_all_certs(cert: X509, cabundle: Vec<Value>) -> Result<Vec<X509>, AttestationError> {
    let mut all_certs = vec![cert];
    for cert in cabundle {
        let cert = (match cert {
            Value::Bytes(b) => Ok(b),
            _ => Err(AttestationError::ParseFailed("cert decode".into())),
        })?;
        let cert = X509::from_der(&cert)
            .map_err(|e| AttestationError::ParseFailed(format!("der: {e}")))?;
        all_certs.push(cert);
    }
    Ok(all_certs)
}

fn verify_cert_chain(
    cert: X509,
    cabundle: Vec<Value>,
    root_cert_pem: Vec<u8>,
) -> Result<(), AttestationError> {
    let certs = get_all_certs(cert, cabundle)?;

    for i in 0..(certs.len() - 1) {
        let pubkey = certs[i + 1]
            .public_key()
            .map_err(|e| AttestationError::ParseFailed(format!("pubkey: {e}")))?;
        if !certs[i]
            .verify(&pubkey)
            .map_err(|e| AttestationError::ParseFailed(format!("signature: {e}")))?
        {
            return Err(AttestationError::VerifyFailed("signature".into()));
        }
        if certs[i + 1].issued(&certs[i]) != X509VerifyResult::OK {
            return Err(AttestationError::VerifyFailed("issuer or subject".into()));
        }
        let current_time =
            Asn1Time::days_from_now(0).map_err(|e| AttestationError::ParseFailed(e.to_string()))?;
        if certs[i].not_after() < current_time || certs[i].not_before() > current_time {
            return Err(AttestationError::VerifyFailed("timestamp".into()));
        }
    }

    let root_cert = X509::from_pem(&root_cert_pem)
        .map_err(|e| AttestationError::ParseFailed(format!("pem: {e}")))?;
    if &root_cert
        != certs
            .last()
            .ok_or(AttestationError::ParseFailed("root".into()))?
    {
        return Err(AttestationError::VerifyFailed("root".into()));
    }
    Ok(())
}

fn parse_attestation_doc(
    attestation_doc: &[u8],
) -> Result<(CoseSign1, BTreeMap<Value, Value>), AttestationError> {
    let cosesign1 = CoseSign1::from_bytes(&attestation_doc)
        .map_err(|e| AttestationError::ParseFailed(format!("cose: {e}")))?;
    let payload = cosesign1
        .get_payload::<Openssl>(None)
        .map_err(|e| AttestationError::ParseFailed(format!("cose payload: {e}")))?;
    let cbor = serde_cbor::from_slice::<Value>(&payload)
        .map_err(|e| AttestationError::ParseFailed(format!("cbor: {e}")))?;
    let attestation_doc = value::from_value::<BTreeMap<Value, Value>>(cbor)
        .map_err(|e| AttestationError::ParseFailed(format!("doc: {e}")))?;

    Ok((cosesign1, attestation_doc))
}

fn parse_pcrs(
    attestation_doc: &mut BTreeMap<Value, Value>,
) -> Result<[[u8; 48]; 3], AttestationError> {
    let pcrs_arr = attestation_doc
        .remove(&"pcrs".to_owned().into())
        .ok_or(AttestationError::ParseFailed("pcrs not found".into()))?;
    let mut pcrs_arr = value::from_value::<BTreeMap<Value, Value>>(pcrs_arr)
        .map_err(|e| AttestationError::ParseFailed(format!("pcrs: {e}")))?;

    let mut result = [[0; 48]; 3];
    for i in 0..3 {
        let pcr = pcrs_arr
            .remove(&(i as u32).into())
            .ok_or(AttestationError::ParseFailed(format!("pcr{i} not found")))?;
        let pcr = (match pcr {
            Value::Bytes(b) => Ok(b),
            _ => Err(AttestationError::ParseFailed(format!(
                "pcr{i} decode failure"
            ))),
        })?;
        result[i] = pcr
            .as_slice()
            .try_into()
            .map_err(|e| AttestationError::ParseFailed(format!("pcr{i} not 48 bytes: {e}")))?;
    }

    Ok(result)
}

fn verify_signature_and_cert_chain(
    attestation_doc: &mut BTreeMap<Value, Value>,
    cosesign1: &CoseSign1,
) -> Result<(), AttestationError> {
    // verify attestation doc signature
    let enclave_certificate = attestation_doc
        .remove(&"certificate".to_owned().into())
        .ok_or(AttestationError::ParseFailed(
            "certificate key not found".to_owned(),
        ))?;
    let enclave_certificate = (match enclave_certificate {
        Value::Bytes(b) => Ok(b),
        _ => Err(AttestationError::ParseFailed(
            "enclave certificate decode failure".to_owned(),
        )),
    })?;
    let enclave_certificate = X509::from_der(&enclave_certificate)
        .map_err(|e| AttestationError::ParseFailed(format!("der: {e}")))?;
    let pub_key = enclave_certificate
        .public_key()
        .map_err(|e| AttestationError::ParseFailed(format!("pubkey: {e}")))?;
    let verify_result = cosesign1
        .verify_signature::<Openssl>(&pub_key)
        .map_err(|e| AttestationError::ParseFailed(format!("signature: {e}")))?;

    if !verify_result {
        return Err(AttestationError::VerifyFailed("signature".into()));
    }

    // verify certificate chain
    let cabundle = attestation_doc
        .remove(&"cabundle".to_owned().into())
        .ok_or(AttestationError::ParseFailed(
            "cabundle key not found in attestation doc".to_owned(),
        ))?;
    let mut cabundle = (match cabundle {
        Value::Array(b) => Ok(b),
        _ => Err(AttestationError::ParseFailed(
            "cabundle decode failure".to_owned(),
        )),
    })?;
    cabundle.reverse();

    let root_cert_pem = include_bytes!("./aws.cert").to_vec();
    verify_cert_chain(enclave_certificate, cabundle, root_cert_pem)?;

    Ok(())
}

fn parse_timestamp(
    attestation_doc: &mut BTreeMap<Value, Value>,
) -> Result<usize, AttestationError> {
    let timestamp = attestation_doc
        .remove(&"timestamp".to_owned().into())
        .ok_or(AttestationError::ParseFailed(
            "timestamp not found in attestation doc".to_owned(),
        ))?;
    let timestamp = (match timestamp {
        Value::Integer(b) => Ok(b),
        _ => Err(AttestationError::ParseFailed(
            "timestamp decode failure".to_owned(),
        )),
    })?;
    let timestamp = timestamp
        .try_into()
        .map_err(|e| AttestationError::ParseFailed(format!("timestamp: {e}")))?;

    Ok(timestamp)
}

fn parse_enclave_key(
    attestation_doc: &mut BTreeMap<Value, Value>,
) -> Result<Vec<u8>, AttestationError> {
    let public_key = attestation_doc
        .remove(&"public_key".to_owned().into())
        .ok_or(AttestationError::ParseFailed(
            "public key not found in attestation doc".to_owned(),
        ))?;
    let public_key = (match public_key {
        Value::Bytes(b) => Ok(b),
        _ => Err(AttestationError::ParseFailed(
            "public key decode failure".to_owned(),
        )),
    })?;

    Ok(public_key)
}

pub fn verify(
    attestation_doc_cbor: Vec<u8>,
    pcrs: [[u8; 48]; 3],
    max_age: usize,
) -> Result<Vec<u8>, AttestationError> {
    // verify attestation and decode fields
    let decoded_data = verify_and_decode_attestation(attestation_doc_cbor)?;

    for i in 0..3 {
        if decoded_data.pcrs[i] != pcrs[i] {
            return Err(AttestationError::VerifyFailed(format!("pcr{i}")));
        }
    }

    // verify age
    let now = Utc::now().timestamp_millis();
    if (now as usize) - max_age > decoded_data.timestamp {
        return Err(AttestationError::VerifyFailed("too old".into()));
    }

    Ok(decoded_data.public_key)
}

pub fn verify_with_timestamp(
    attestation_doc_cbor: Vec<u8>,
    pcrs: [[u8; 48]; 3],
    timestamp: usize,
) -> Result<Vec<u8>, AttestationError> {
    // verify attestation and decode fields
    let decoded_data = verify_and_decode_attestation(attestation_doc_cbor)?;

    for i in 0..3 {
        if decoded_data.pcrs[i] != pcrs[i] {
            return Err(AttestationError::VerifyFailed(format!("pcr{i}")));
        }
    }

    // verify timestamp
    if timestamp != decoded_data.timestamp {
        return Err(AttestationError::VerifyFailed(
            "timestamp does not match".into(),
        ));
    }

    Ok(decoded_data.public_key)
}

pub async fn get_attestation_doc(endpoint: Uri) -> Result<Vec<u8>, AttestationError> {
    let client = Client::builder(TokioExecutor::new()).build_http::<Full<Bytes>>();
    let res = client.get(endpoint).await?;
    let body = res.collect().await?.to_bytes();

    Ok(body.to_vec())
}

pub fn decode_attestation(
    attestation_doc: Vec<u8>,
) -> Result<AttestationDecoded, AttestationError> {
    let mut result = AttestationDecoded {
        pcrs: [[0; 48]; 3],
        timestamp: 0,
        public_key: Vec::new(),
    };

    // parse attestation doc
    let (_, mut attestation_doc) = parse_attestation_doc(&attestation_doc)?;

    // parse pcrs
    result.pcrs = parse_pcrs(&mut attestation_doc)?;

    // parse timestamp
    result.timestamp = parse_timestamp(&mut attestation_doc)?;

    // parse the enclave key
    result.public_key = parse_enclave_key(&mut attestation_doc)?;

    Ok(result)
}

pub fn verify_and_decode_attestation(
    attestation_doc: Vec<u8>,
) -> Result<AttestationDecoded, AttestationError> {
    let mut result = AttestationDecoded {
        pcrs: [[0; 48]; 3],
        timestamp: 0,
        public_key: Vec::new(),
    };

    // parse attestation doc
    let (cosesign1, mut attestation_doc) = parse_attestation_doc(&attestation_doc)?;

    // parse pcrs
    result.pcrs = parse_pcrs(&mut attestation_doc)?;

    // verify signature and cert chain
    verify_signature_and_cert_chain(&mut attestation_doc, &cosesign1)?;

    // parse timestamp
    result.timestamp = parse_timestamp(&mut attestation_doc)?;

    // return the enclave key
    result.public_key = parse_enclave_key(&mut attestation_doc)?;

    Ok(result)
}
