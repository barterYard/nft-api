use std::collections::HashMap;

use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Owner {
    pub _id: ObjectId,
    pub address: String,
}

impl Owner {
    pub fn new(address: String) -> Self {
        Owner {
            _id: ObjectId::new(),
            address,
        }
    }

    pub async fn get_or_create(
        client: &Client,
        address: String,
        session: Option<&mut ClientSession>,
    ) -> Self {
        let owner_col = Owner::get_collection(client);
        let address = match address.as_str() {
            "null" => "0x0".to_string(),
            _ => address,
        };
        match owner_col
            .find_one(mongo_doc! {"address": &address}, None)
            .await
        {
            Ok(Some(owner)) => owner,
            _ => {
                let new_owner = Owner::new(address);
                let res = match session {
                    Some(s) => owner_col.insert_one_with_session(&new_owner, None, s).await,
                    _ => owner_col.insert_one(&new_owner, None).await,
                };
                if res.is_err() {
                    println!("owner {:?}", res.err());
                }
                new_owner
            }
        }
    }

    pub async fn update(
        &self,
        operation: Document,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let o_col = Owner::get_collection(client);
        let q = mongo_doc! {"_id": self._id};
        match session {
            Some(s) => o_col.update_one_with_session(q, operation, None, s).await,
            _ => o_col.update_one(q, operation, None).await,
        }
    }
}
