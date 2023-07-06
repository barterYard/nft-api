use std::{collections::HashMap, env};

use serde::{self, Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "contracts")]
    pub contracts: Option<HashMap<String, Contract>>,

    #[serde(rename = "accounts")]
    pub accounts: Option<HashMap<String, Account>>,
}

impl Config {
    pub fn parse(file: &'static str) -> Config {
        serde_json::from_str(file).expect("JSON was not well-formatted")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(rename = "address")]
    pub address: String,

    #[serde(rename = "key")]
    account_key: Option<serde_json::Value>,
}

impl Account {
    pub fn key(self) -> Key {
        let mut key = match self.account_key {
            Some(key) => match key.is_string() {
                true => Key {
                    key_type: "hex".to_string(),
                    index: 0,
                    private_key: serde_json::from_value(key).expect("string"),
                    signature_algorithm: "ECDSA_secp256k1".to_string(),
                    hash_algorithm: "SHA3_256".to_string(),
                },
                false => serde_json::from_value(key).expect("key was not well-formatted"),
            },
            _ => panic!("No key associated with account {}", self.address),
        };
        if key.private_key.starts_with("$") {
            let env_var = key.private_key.get(1..).unwrap();
            key.private_key = env::var(env_var.to_string())
                .expect(format!("{} not found in env", env_var.to_string()).as_str());
        }
        key
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Key {
    #[serde(rename = "type")]
    pub key_type: String,

    #[serde(rename = "index")]
    pub index: i64,

    #[serde(rename = "privateKey")]
    pub private_key: String,

    #[serde(rename = "signatureAlgorithm")]
    pub signature_algorithm: String,

    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contract {
    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "aliases")]
    pub aliases: HashMap<String, String>,
}
