use crate::mongo::models::common::ModelCollection;
use bson::DateTime;
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Contract {
    pub _id: bson::oid::ObjectId,
    pub address: String,
    pub id: String,
    pub locked: bool,
    pub deleted: bool,
    pub identifier: String,
    pub deployments: Vec<Deployment>,
    #[serde(rename = "type")]
    pub contract_type: String,
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
