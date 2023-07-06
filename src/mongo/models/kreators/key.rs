use crate::mongo::models::{common::ModelCollection, mongo_doc};
use mongodb::Client;
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ModelCollection)]
pub struct Key {
    pub _id: bson::oid::ObjectId,
    pub key: String,
    pub refresh: Option<String>,
    pub user_id: String,
}

impl Key {
    pub async fn get_by_user_id(client: &Client, user_id: String) -> Option<Key> {
        let key_db = Key::get_collection(client);
        if let Ok(Some(key)) = key_db.find_one(mongo_doc! {"user_id": user_id}, None).await {
            Some(key)
        } else {
            None
        }
    }
}
