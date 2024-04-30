use ecies::{decrypt, encrypt};
use openssl::rand;
use openssl::rand::rand_bytes;
use openssl::symm;
use openssl::symm::{Cipher, Crypter, Mode};
use std::error::Error;

pub struct SecretData {
    #[allow(unused)]
    encrypted_data: Vec<u8>,
    #[allow(unused)]
    acl_data: Vec<u8>,
}

#[allow(unused)]
pub fn decrypt_ecies(receiver_priv: &[u8], msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let result = decrypt(receiver_priv, msg).unwrap();
    Ok(result)
}

#[allow(unused)]
pub fn encrypt_ecies(receiver_pub: &[u8], msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let result = encrypt(receiver_pub, msg).unwrap();
    Ok(result)
}

pub fn decrypt_aes(encrypted_data: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if encrypted_data.len() <= 16 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid encrypted data format.",
        )));
    }

    let iv = &encrypted_data[0..16];
    let encrypted_text = &encrypted_data[16..];

    let cipher = Cipher::aes_256_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, secret_key, Some(iv))?;
    let mut decrypted = vec![0; encrypted_text.len() + cipher.block_size()];
    let count = decrypter.update(encrypted_text, &mut decrypted)?;
    let rest = decrypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest);

    Ok(decrypted)
}

#[allow(unused)]
pub fn decrypt_aes_gcm(
    encrypted_data: &[u8],
    secret_key: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    let iv_length = 12; // GCM recommends a 12-byte IV
    let auth_tag_length = 16; // GCM uses a 16-byte authentication tag

    if encrypted_data.len() <= (iv_length + auth_tag_length) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid encrypted data format.",
        )));
    }

    let iv = &encrypted_data[0..iv_length];
    let encrypted_text = &encrypted_data[iv_length..encrypted_data.len() - auth_tag_length];
    let auth_tag = &encrypted_data[encrypted_data.len() - auth_tag_length..];

    let cipher = Cipher::aes_256_gcm();
    let mut decrypter = symm::Crypter::new(cipher, Mode::Decrypt, secret_key, Some(iv))?;
    decrypter.set_tag(auth_tag)?;

    let mut decrypted = vec![0; encrypted_text.len() + cipher.block_size()];
    let count = decrypter.update(encrypted_text, &mut decrypted)?;
    let rest = decrypter.finalize(&mut decrypted[count..])?;
    decrypted.truncate(count + rest);

    Ok(decrypted)
}

pub fn encrypt_aes(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = symm::Cipher::aes_256_cbc();
    let mut iv = vec![0; 16];
    rand::rand_bytes(&mut iv)?;
    let mut encrypter = symm::Crypter::new(cipher, symm::Mode::Encrypt, key, Some(&iv))?;
    let mut encrypted = vec![0; data.len() + cipher.block_size()];
    let count = encrypter.update(data, &mut encrypted)?;
    let rest = encrypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest);

    // Concatenate the IV and encrypted data into a single Vec<u8>
    iv.extend_from_slice(&encrypted);
    Ok(iv)
}

#[allow(unused)]
pub fn encrypt_aes_gcm(data: &[u8], key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = symm::Cipher::aes_256_gcm();
    let mut iv = vec![0; 12]; // GCM recommends a 12-byte IV
    rand::rand_bytes(&mut iv)?;
    let mut encrypter = symm::Crypter::new(cipher, symm::Mode::Encrypt, key, Some(&iv))?;
    let mut encrypted = vec![0; data.len() + cipher.block_size()];
    let count = encrypter.update(data, &mut encrypted)?;
    let rest = encrypter.finalize(&mut encrypted[count..])?;
    encrypted.truncate(count + rest);

    // Append the authentication tag to the encrypted data
    let mut auth_tag = vec![0; 16];
    encrypter.get_tag(&mut auth_tag)?;

    // Concatenate the IV, encrypted data, and authentication tag into a single Vec<u8>
    iv.extend_from_slice(&encrypted);
    iv.extend_from_slice(&auth_tag);

    Ok(iv)
}

#[allow(unused)]
pub fn encrypt_data_with_ecies_and_aes(
    receiver_pub: &[u8],
    data: &[u8],
) -> Result<SecretData, Box<dyn std::error::Error>> {
    let mut key = vec![0u8; 32];
    rand_bytes(&mut key)?;

    let encrypted_data = encrypt_aes(data, &key).unwrap();

    let encrypted_secret_key = encrypt(receiver_pub, &key).unwrap();

    Ok(SecretData {
        encrypted_data,
        acl_data: encrypted_secret_key,
    })
}

pub fn decrypt_data_with_ecies_and_aes(
    encrypted_data: &[u8],
    acl_data: &[u8],
    private_key: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let decrypted_secret_key = decrypt(private_key, acl_data);
    match decrypted_secret_key {
        Ok(secret_key) => {
            let decrypted_data = try_decrypt(encrypted_data, &secret_key).unwrap();

            Ok(decrypted_data)
        }
        Err(_) => Err("Invalid ECIES key used, skipping the ASK".into()),
    }
}

pub fn try_decrypt(encrypted_data: &[u8], secret_key: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    decrypt_aes_gcm(encrypted_data, secret_key).or_else(|_| decrypt_aes(encrypted_data, secret_key))
}

#[cfg(test)]
mod tests {
    use ecies::{PublicKey, SecretKey};
    use hex;

    use super::{
        decrypt_aes, decrypt_aes_gcm, decrypt_ecies, encrypt_aes, encrypt_aes_gcm, encrypt_ecies,
    };

    #[test]
    fn test_key() {
        let private_key_string = "ca9cbf143a43e422a307b03ec61a82ce99c053290c3053655d0ad69e863a18c4";
        let private_key = hex::decode(private_key_string).unwrap();
        let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
        let sk = SecretKey::parse(&private_key).unwrap();

        let public_key = PublicKey::from_secret_key(&sk);
        let public_key = public_key.serialize_compressed();

        let decoded_vec =
            hex::decode("039e61f849ca3a47c90bbb2d387e612e85c54c460286ddbf6ee339526c3dbdee39")
                .expect("Failed to decode hex string");

        // Convert Vec<u8> to [u8; 33]
        let expected_key: [u8; 33] = decoded_vec.try_into().expect("Length mismatch");

        assert_eq!(public_key, expected_key);
    }

    #[test]
    fn test_cipher_enc() {
        let private_key_string = "ca9cbf143a43e422a307b03ec61a82ce99c053290c3053655d0ad69e863a18c4";
        let private_key = hex::decode(private_key_string).unwrap();
        let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();
        let sk = SecretKey::parse(&private_key).unwrap();

        let public_key = PublicKey::from_secret_key(&sk);
        let public_key = public_key.serialize_compressed();

        let cipher = "0000111100001111000011110000111100001111000011110000111100001111";
        let cipher = hex::decode(cipher).unwrap();
        let encrypted_cipher = encrypt_ecies(&public_key, &cipher).unwrap();

        let encrypted_cipher = hex::encode(encrypted_cipher).to_string();
        dbg!(encrypted_cipher);
    }

    #[test]
    fn test_cipher_dec() {
        let encrypted_cipher = "0438f743cfbb1f980640ccdbe156f84ebccbf7eb10423e5b7a29ef0efc1d41e5bfa2484e95cf6d819c7778d06cb9ea0461ea3fa0d9119fc41f6fe80e66e9feb37fa08d9cd5524956bd7b718d6ffb9f74636336daa6590a05d1063ee24f1d6db0cc78c84e5337f3cbd4a865a90ea1e047a0e2ae5beeb2576c00e4cae8363d051b71";
        let encrypted_cipher = hex::decode(encrypted_cipher).unwrap();
        let private_key_string = "ca9cbf143a43e422a307b03ec61a82ce99c053290c3053655d0ad69e863a18c4";
        let private_key = hex::decode(private_key_string).unwrap();
        let private_key: &[u8; 32] = private_key.as_slice().try_into().unwrap();

        let cipher = decrypt_ecies(private_key, &encrypted_cipher).unwrap();
        let cipher = hex::encode(cipher).to_string();
        let expected = "0000111100001111000011110000111100001111000011110000111100001111";

        assert_eq!(cipher, expected);
    }

    #[test]
    fn test_aes_enc() {
        let data = b"this is the data that we wish to encrypt";

        let cipher = "0000111100001111000011110000111100001111000011110000111100001111";
        let cipher = hex::decode(cipher).unwrap();

        let encrypted_aes_data = encrypt_aes(data, &cipher).unwrap();

        let encrypted_aes_data = hex::encode(encrypted_aes_data).to_string();

        dbg!(encrypted_aes_data);
    }

    #[test]
    fn test_aes_dec() {
        let expected_data = b"this is the data that we wish to encrypt";
        let encrypted_data = "8b276aad40ba5572ec11516388b0ab3dec11ae4ce6488a7dfad93a4f40429befe58098a6b2a4a316d6654fd14f7eac8c9517046312a9b659d9902bbff41e75fe";
        let encrypted_data = hex::decode(encrypted_data).unwrap();

        let cipher = "0000111100001111000011110000111100001111000011110000111100001111";
        let cipher = hex::decode(cipher).unwrap();

        let fetched_data = decrypt_aes(&encrypted_data, &cipher).unwrap();

        let expected_data = hex::encode(expected_data).to_string();
        let fetched_data = hex::encode(fetched_data).to_string();
        assert_eq!(expected_data, fetched_data);
    }

    #[test]
    fn test_aes_gcm_enc() {
        let data = b"this is the data that we wish to encrypt";

        let cipher = "0000111100001111000011110000111100001111000011110000111100001111";
        let cipher = hex::decode(cipher).unwrap();

        let encrypted_aes_data = encrypt_aes_gcm(data, &cipher).unwrap();

        let encrypted_aes_data = hex::encode(encrypted_aes_data).to_string();

        dbg!(encrypted_aes_data);
    }

    #[test]
    fn test_aes_gcm_dec() {
        let expected_data = b"this is the data that we wish to encrypt";
        let encrypted_data = "727bae219a76934de1e699b929ff5762ee640dbc8ad2357b55bab6bb0b81428a2614a4c0b949185aa8b55e468e34446a52113b61752ab0c8487a374942c35c34b117eb6c";
        let encrypted_data = hex::decode(encrypted_data).unwrap();

        let cipher = "0000111100001111000011110000111100001111000011110000111100001111";
        let cipher = hex::decode(cipher).unwrap();

        let fetched_data = decrypt_aes_gcm(&encrypted_data, &cipher).unwrap();

        let expected_data = hex::encode(expected_data).to_string();
        let fetched_data = hex::encode(fetched_data).to_string();
        assert_eq!(expected_data, fetched_data);
    }
}
