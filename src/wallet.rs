use base64::{decode, encode};
use rsa::{traits::PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};



//symetric

use std::{str};


#[derive(Debug)]
pub struct Wallet {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
}

// Define a struct for serializing and deserializing the public key
#[derive(Serialize, Deserialize)]
struct SerializedPublicKey {
    modulus: String,
    exponent: String,
}

impl From<RsaPublicKey> for SerializedPublicKey {
    fn from(key: RsaPublicKey) -> Self {
        SerializedPublicKey {
            modulus: encode(key.n().to_bytes_le()),
            exponent: encode(key.e().to_bytes_le()),
        }
    }
}
impl From<SerializedPublicKey> for RsaPublicKey {
    fn from(serialized: SerializedPublicKey) -> Self {
        let modulus_bytes = decode(&serialized.modulus).expect("Failed to decode modulus bytes");
        let exponent_bytes = decode(&serialized.exponent).expect("Failed to decode exponent bytes");

        let x = RsaPublicKey::new(
            rsa::BigUint::from_bytes_le(&modulus_bytes),
            rsa::BigUint::from_bytes_le(&exponent_bytes),
        );
        x.unwrap()
    }
}
impl Wallet {
    pub fn new() -> Self {
        let priv_key = RsaPrivateKey::new(&mut rand::thread_rng(), 128)
            .expect("Failed to generate a private key");
        Self {
            private_key: priv_key.clone(),
            public_key: RsaPublicKey::from(&priv_key),
        }
    }
    pub fn pubkey_as_string(&self) -> String {
        let serialized_key: SerializedPublicKey = self.public_key.clone().into();
        serde_json::to_string(&serialized_key).unwrap()
    }
}
