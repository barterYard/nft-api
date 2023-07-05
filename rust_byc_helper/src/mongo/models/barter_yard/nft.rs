use proc::ModelCollection;
use serde::{Deserialize, Serialize};

use crate::mongo::models::common::ModelCollection;
#[derive(Serialize, Deserialize, Debug, ModelCollection)]
pub struct Nft {
    pub _id: bson::oid::ObjectId,
    pub token_id: i64,
    pub cid: String,
    pub attributes: Vec<bson::Document>,
    pub name: String,
    pub description: String,
    pub score: f64,
    pub rank: i64,
}
// barter_yard_db!(Nft, "nfts");
