use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

use super::contract::Contract;

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Nft {
    pub _id: ObjectId,
    pub id: i64,
    pub description: Option<String>,
    pub name: Option<String>,
    pub burned: bool,
    pub owner: ObjectId,
    pub contract_id: String,
    pub contract: ObjectId,
}

impl Nft {
    pub async fn insert(&self, client: &Client, session: Option<&mut ClientSession>) {
        let nft_col = Nft::get_collection(client);
        match nft_col
            .find_one(
                mongo_doc! {"contract": self.contract, "id": self.id.clone()},
                None,
            )
            .await
        {
            Ok(Some(_x)) => {}
            _ => {
                if session.is_some() {
                    let _ = nft_col
                        .insert_one_with_session(self, None, session.unwrap())
                        .await;
                }
                let _ = nft_col.insert_one(self, None).await;
            }
        }
    }

    pub async fn get_or_create(
        client: &Client,
        contract: &Contract,
        nft_id: i64,
        save: bool,
        session: Option<&mut ClientSession>,
    ) -> (Nft, bool) {
        let nft_col = Nft::get_collection(client);
        match nft_col
            .find_one(mongo_doc! {"contract": contract._id, "id": nft_id}, None)
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
                    let _ = match session {
                        Some(s) => nft_col.insert_one_with_session(&new_nft, None, s).await,
                        _ => nft_col.insert_one(&new_nft, None).await,
                    };
                }

                (new_nft, true)
            }
        }
    }

    pub async fn burn(
        &self,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        self.update(
            mongo_doc! {
                    "$set": mongo_doc! {
                    "burned": true,
                }
            },
            client,
            session,
        )
        .await
    }

    pub async fn update(
        &self,
        update: Document,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        if !update.contains_key("$set") {
            panic!("don't use this method to replace document");
        }
        let nft_col = Nft::get_collection(client);
        let q = mongo_doc! {
            "_id": self._id
        };
        match session {
            Some(s) => nft_col.update_one_with_session(q, update, None, s).await,
            _ => nft_col.update_one(q, update, None).await,
        }
    }
    pub async fn mint(
        &self,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let op = mongo_doc! {"$set": mongo_doc! {
            "burned": false,
        }};
        self.update(op, client, session).await
    }
}
