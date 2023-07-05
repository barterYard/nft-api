use std::collections::HashMap;

use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
    Client,
};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

use super::contract;

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Owner {
    pub _id: ObjectId,
    pub address: String,
    pub nfts: Option<HashMap<String, Vec<ObjectId>>>,
}

impl Owner {
    pub fn new(address: String, nfts: Option<HashMap<String, Vec<ObjectId>>>) -> Self {
        Owner {
            _id: ObjectId::new(),
            address,
            nfts,
        }
    }
    pub async fn update(&self, client: &Client) -> Result<UpdateResult, Error> {
        let nfts: Document = self
            .nfts
            .clone()
            .unwrap()
            .into_iter()
            .map(|(k, x)| {
                (
                    k,
                    bson::Bson::Array(x.into_iter().map(|c| bson::Bson::ObjectId(c)).collect()),
                )
            })
            .collect();
        Owner::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                mongo_doc! {"$set": mongo_doc! {
                    "nfts": nfts,
                }},
                None,
            )
            .await
    }

    pub async fn get_or_create(client: &Client, address: String) -> Self {
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
                    let _ = owner_col.insert_one(&new_owner, None).await;
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
    ) -> Result<UpdateResult, Error> {
        if self.nfts.is_none() {
            let mut nfts = HashMap::new();
            nfts.insert(contract_id, vec![nft]);
            self.nfts = Some(nfts)
        } else {
            match self.nfts.as_mut().unwrap().get_mut(&contract_id) {
                Some(x) => x.push(nft),
                _ => {
                    self.nfts.as_mut().unwrap().insert(contract_id, vec![nft]);
                    {}
                }
            };
        }
        self.update(client).await
    }

    pub async fn remove_owned_nft(
        &mut self,
        contract_id: String,
        nft: ObjectId,
        client: &Client,
    ) -> Result<UpdateResult, Error> {
        if self.nfts.is_none() {
            let mut nfts = HashMap::new();
            nfts.insert(contract_id.clone(), vec![nft]);
            self.nfts = Some(nfts)
        } else {
            match self.nfts.as_ref().unwrap().get(&contract_id) {
                Some(_x) => {
                    let new_nfts = self.nfts.clone().unwrap();
                    let mut doc = new_nfts.get(&contract_id).unwrap().clone();

                    doc = doc
                        .into_iter()
                        .filter(|x| x.to_string() != nft.to_string())
                        .collect();
                    self.nfts.as_mut().unwrap().insert(contract_id.clone(), doc);
                }
                _ => {}
            };
        }

        self.update(client).await
    }
}
