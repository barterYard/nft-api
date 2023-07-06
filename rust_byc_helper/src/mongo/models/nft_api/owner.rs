use std::collections::HashMap;

use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use log::info;
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Owner {
    pub _id: ObjectId,
    pub address: String,
    pub nfts: HashMap<String, Vec<ObjectId>>,
}

impl Owner {
    pub fn new(address: String, nfts: Option<HashMap<String, Vec<ObjectId>>>) -> Self {
        Owner {
            _id: ObjectId::new(),
            address,
            nfts: nfts.unwrap_or_default(),
        }
    }

    pub async fn get_or_create(
        client: &Client,
        address: String,
        session: Option<&mut ClientSession>,
    ) -> Self {
        let owner_col = Owner::get_collection(client);
        let mut address = address.clone();
        if address == "null" {
            address = "0x0".to_string();
        }
        match owner_col
            .find_one(mongo_doc! {"address": &address}, None)
            .await
        {
            Ok(y) => match y {
                Some(owner) => return owner,
                _ => {
                    let new_owner = Owner {
                        address,
                        _id: bson::oid::ObjectId::new(),
                        ..Default::default()
                    };
                    if session.is_some() {
                        let _ =
                            owner_col.insert_one_with_session(&new_owner, None, session.unwrap());
                    } else {
                        let _ = owner_col.insert_one(&new_owner, None).await;
                    }
                    new_owner
                }
            },
            Err(_) => {
                let new_owner = Owner {
                    address,
                    _id: bson::oid::ObjectId::new(),
                    ..Default::default()
                };
                let _ = owner_col.insert_one(&new_owner, None).await;
                new_owner
            }
        }
    }

    pub async fn add_owned_nft(
        &mut self,
        nft: ObjectId,
        contract_id: String,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let field_name = "nfts.".to_owned() + &contract_id.replace(".", "_");
        if session.is_some() {
            Owner::get_collection(client)
                .update_one_with_session(
                    mongo_doc! {"_id": self._id},
                    mongo_doc! {
                        "$addToSet": {field_name: nft}
                    },
                    None,
                    session.unwrap(),
                )
                .await
        } else {
            Owner::get_collection(client)
                .update_one(
                    mongo_doc! {"_id": self._id},
                    mongo_doc! {
                        "$addToSet": {field_name: nft}
                    },
                    None,
                )
                .await
        }
    }

    pub async fn remove_owned_nft(
        &mut self,
        contract_id: String,
        nft: ObjectId,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let field_name = "nfts.".to_owned() + &contract_id.replace(".", "_");
        if session.is_some() {
            Owner::get_collection(client)
                .update_one_with_session(
                    mongo_doc! {"_id": self._id},
                    mongo_doc! {
                        "$pull": {field_name: nft}
                    },
                    None,
                    session.unwrap(),
                )
                .await
        } else {
            Owner::get_collection(client)
                .update_one(
                    mongo_doc! {"_id": self._id},
                    mongo_doc! {
                        "$pull": {field_name: nft}
                    },
                    None,
                )
                .await
        }
    }
}
