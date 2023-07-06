use crate::{barter_yard_db, mongo::models::common::ModelCollection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FloorPrice {
    pub price: f64,
    #[serde(rename = "type")]
    pub ft_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Listing {
    pub price: f64,
    pub nft_id: u64,
    pub ft_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Floor {
    pub _id: bson::oid::ObjectId,
    pub collection: String,
    pub floor: FloorPrice,
    pub listings: Vec<Listing>,
}
barter_yard_db!(Floor, "floor_price");
