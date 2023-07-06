use std::{collections::HashMap, str::FromStr};

use self::{config::Config, network::get_script};
use async_trait::async_trait;
use flow_sdk::{
    access::TransactionResultResponse,
    account::Account,
    algorithms::{self},
    error::TonicError,
    prelude::{cadence_json::ValueOwned, hex, TonicHyperFlowClient},
    transaction::TransactionHeaderBuilder,
};
pub mod config;
pub mod network;

#[async_trait(?Send)]
pub trait Scripts {
    fn get(&self) -> String;
    fn get_config() -> &'static str;
    async fn execute(
        &self,
        args: Vec<ValueOwned>,
    ) -> Result<flow_sdk::access::ExecuteScriptResponse, flow_sdk::error::TonicError> {
        let script = get_script(self.get(), Self::get_config());
        let net = network::FlowNetwork::get();
        let mut client = net.get_flow_client().await;
        client.execute_script_at_latest_block(script, args).await
    }
}
pub struct GenericTransaction {
    pub data: String,
}

impl Transactions for GenericTransaction {
    fn get(&self) -> String {
        self.data.clone()
    }

    fn get_config() -> &'static str {
        ""
    }
}
#[async_trait(?Send)]
pub trait Transactions {
    fn get(&self) -> String;
    fn get_config() -> &'static str;

    async fn execute(
        &self,
        signer_address: String,
        key: String,
        args: HashMap<String, String>,
    ) -> Result<Option<TransactionResultResponse>, TonicError> {
        let script = self.get().to_string();
        let net = network::FlowNetwork::get();
        let client = net.get_flow_client().await;
        let addr = hex::decode(signer_address.clone()).unwrap();

        let secret_key = algorithms::secp256k1::SecretKey::from_str(key.as_str()).unwrap();

        let mut acc = Account::<TonicHyperFlowClient>::new(client, addr, secret_key)
            .await
            .expect("Invalid Account keys");
        let header = TransactionHeaderBuilder::new()
            .script_owned(script)
            .arguments(args)
            .build();
        let c = acc.send_transaction_header(&header).await;

        c.unwrap().finalize(acc.client()).await
    }

    async fn execute_local(
        &self,
        signer: &'static str,
        args: Vec<ValueOwned>,
    ) -> Result<Option<TransactionResultResponse>, TonicError> {
        let config = Config::parse(Self::get_config());
        let script = get_script(self.get(), Self::get_config());
        let net = network::FlowNetwork::get();
        let client = net.get_flow_client().await;
        let account = config.accounts.unwrap().get(signer).unwrap().clone();
        let addr = hex::decode(account.address.clone()).unwrap();

        let secret_key =
            algorithms::secp256k1::SecretKey::from_str(account.clone().key().private_key.as_str())
                .unwrap();

        let mut acc = Account::<TonicHyperFlowClient>::new(client, addr, secret_key)
            .await
            .expect("Invalid Account keys");

        let header = TransactionHeaderBuilder::new()
            .script_owned(script)
            .arguments(args)
            .build();
        let c = acc.send_transaction_header(&header).await;

        c.unwrap().finalize(acc.client()).await
    }
}
