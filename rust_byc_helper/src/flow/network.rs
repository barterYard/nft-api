use flow_sdk::prelude::TonicHyperFlowClient;
use std::env;

use crate::flow::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNetwork {
    Testnet,
    Mainnet,
}

impl FlowNetwork {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
        }
    }
    fn from_string(network: String) -> FlowNetwork {
        match network.as_str() {
            "mainnet" => FlowNetwork::Mainnet,
            _ => FlowNetwork::Testnet,
        }
    }

    pub fn get() -> FlowNetwork {
        FlowNetwork::from_string(env::var("FLOW_ENV").unwrap_or_else(|_| "testnet".to_string()))
    }

    pub async fn get_flow_client(&self) -> TonicHyperFlowClient {
        match self {
            Self::Mainnet => TonicHyperFlowClient::mainnet().await.unwrap(),
            Self::Testnet => TonicHyperFlowClient::testnet().await.unwrap(),
        }
    }
}

pub fn get_script(script: String, config: &'static str) -> String {
    let config = Config::parse(config);
    let contracts = config.contracts.unwrap();
    let mut script_string = script.to_string();
    let network = FlowNetwork::get().as_str();
    let imports = script.lines().filter(|l| l.contains("import"));

    for import in imports {
        let import_line: Vec<&str> = import.trim().split(' ').collect::<Vec<&str>>();
        if import_line.len() == 4 {
            let contract = contracts.get(import_line[1]).unwrap();
            let path = import_line[3];
            let to = contract.aliases.get(network).unwrap().as_str();
            let import_fin = match to.starts_with("0x") {
                true => import.replace(path, to),
                false => import.replace(path, &["0x", to].join("")),
            };
            script_string = script_string.replace(import, import_fin.as_str());
        }
    }
    script_string
}
