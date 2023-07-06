use crate::mongo::models::{common::ModelCollection, mongo_doc};

use mongodb::Client;
use proc::ModelCollection;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, ModelCollection)]
pub struct Barter {
    pub _id: bson::oid::ObjectId,
    #[serde(rename = "barterID")]
    pub barter_id: i32,
}

impl Barter {
    pub async fn get_by_id(client: &Client, id: String) -> Option<Barter> {
        let barters = Barter::get_collection(client);
        match barters
            .find_one(mongo_doc! { "barterID": id.parse::<i32>().unwrap()}, None)
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
