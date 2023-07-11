use crate::mongo::models::common::ModelCollection;
use crate::mongo::models::mongo_doc;
use bson::{DateTime, Document};
use mongodb::Client;
use proc::ModelCollection;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Contract {
    pub _id: bson::oid::ObjectId,
    pub address: String,
    pub id: String,
    pub locked: bool,
    pub deleted: bool,
    pub done: bool,
    pub identifier: String,
    #[serde(rename = "lastCursor")]
    pub last_cursor: Option<String>,
    pub deployments: Vec<Deployment>,
    #[serde(rename = "type")]
    pub contract_type: String,
}

impl Contract {
    pub async fn update(&self, client: &Client, update: Document) {
        let _ = Contract::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                update,
                None,
            )
            .await;
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deployment {
    pub time: DateTime,
}
impl Default for Deployment {
    fn default() -> Self {
        Self {
            time: DateTime::now(),
        }
    }
}
