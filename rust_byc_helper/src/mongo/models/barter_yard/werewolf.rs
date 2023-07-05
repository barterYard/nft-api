use std::str::FromStr;

use crate::{
    barter_yard_db,
    mongo::models::{common::ModelCollection, mongo_doc},
};
use bson::oid::ObjectId;
use mongodb::Client;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Werewolf {
    pub _id: bson::oid::ObjectId,
    pub address: String,
    pub nfts: Option<Vec<i64>>,
    pub dapper_address: Option<String>,
    pub cid: Option<String>,
    #[serde(rename = "walletType")]
    pub wallet_type: Option<String>,
}
barter_yard_db!(Werewolf, "werewolves");

impl Werewolf {
    pub async fn get_by_address(client: &Client, address: String) -> Option<Self> {
        let ww = Werewolf::get_collection(client);
        match ww.find_one(mongo_doc! { "address": address}, None).await {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }
    pub async fn get_by_id(client: &Client, id: String) -> Option<Self> {
        let ww = Werewolf::get_collection(client);
        match ww
            .find_one(mongo_doc! { "_id": ObjectId::from_str(&id).unwrap()}, None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }
}
