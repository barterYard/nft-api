use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;
use mongodb::{error::Error, results::UpdateResult, Client};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

use super::contract::Contract;

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Nft {
    pub _id: ObjectId,
    pub id: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub burned: bool,
    pub contract_id: String,
    pub contract: ObjectId,
}

impl Nft {
    pub async fn insert(&self, client: &Client) {
        let nft_col = Nft::get_collection(client);
        match nft_col
            .find_one(
                mongo_doc! {"contract": self.contract, "id": self.id.clone()},
                None,
            )
            .await
        {
            Ok(Some(x)) => {}
            _ => {
                let _ = nft_col.insert_one(self, None).await;
            }
        }
    }

    pub async fn get_or_create(
        client: &Client,
        contract: &Contract,
        nft_id: String,
        save: bool,
    ) -> (Nft, bool) {
        let nft_col = Nft::get_collection(client);
        match nft_col
            .find_one(
                mongo_doc! {"contract": contract._id, "id": nft_id.clone(), "burned": false},
                None,
            )
            .await
        {
            Ok(Some(nft)) => return (nft, false),
            _ => {
                let new_nft = Nft {
                    contract: contract._id,
                    contract_id: contract.id.clone(),
                    id: nft_id,
                    _id: bson::oid::ObjectId::new(),
                    ..Default::default()
                };
                if save {
                    let _ = nft_col.insert_one(&new_nft, None).await;
                }
                (new_nft, true)
            }
        }
    }

    pub async fn burn(&self, client: &Client) -> Result<UpdateResult, Error> {
        Nft::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                mongo_doc! {"$set": mongo_doc! {
                    "burned": true,
                }},
                None,
            )
            .await
    }
    pub async fn mint(&self, client: &Client) -> Result<UpdateResult, Error> {
        Nft::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                mongo_doc! {"$set": mongo_doc! {
                    "burned": false,
                }},
                None,
            )
            .await
    }
}
