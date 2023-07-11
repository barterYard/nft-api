use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;
use log::{error, info};
use mongodb::{error::Error, results::InsertOneResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Transfer {
    pub _id: ObjectId,
    pub date: String,
    pub from: String,
    pub to: String,
    pub nft: Option<ObjectId>,
    pub nft_id: i64,
    pub contract: ObjectId,
}

impl Transfer {
    pub fn new(
        date: String,
        from: String,
        to: String,
        contract: ObjectId,
        nft: Option<ObjectId>,
        nft_id: i64,
    ) -> Self {
        Transfer {
            _id: ObjectId::new(),
            date,
            from,
            to,
            nft,
            nft_id,
            contract,
        }
    }

    pub async fn create(
        date: String,
        from: String,
        to: String,
        nft: Option<ObjectId>,
        nft_id: i64,
        contract: ObjectId,
        client: &Client,
    ) -> Result<InsertOneResult, Error> {
        let transfer = Transfer {
            _id: ObjectId::new(),
            date,
            from,
            to,
            nft,
            nft_id,
            contract,
        };
        Transfer::get_collection(client)
            .insert_one(transfer, None)
            .await
    }

    pub async fn find(
        date: String,
        from: String,
        to: String,
        nft: ObjectId,
        client: &Client,
    ) -> Option<Transfer> {
        let transfer_col = Transfer::get_collection(client);

        match transfer_col
            .find_one(
                mongo_doc! {
                    "date": date.clone(),
                    "from": from.clone(),
                    "to": to.clone(),
                    "nft": nft.clone()
                },
                None,
            )
            .await
        {
            Ok(Some(c)) => Some(c),
            Err(e) => {
                error!("{:?}", e);
                return None;
            }
            _ => None,
        }
    }

    pub async fn get_or_create(
        date: String,
        from: String,
        to: String,
        nft_id: i64,

        contract: ObjectId,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Option<(Transfer, bool)> {
        let transfer_col = Transfer::get_collection(client);

        match transfer_col
            .find_one(
                mongo_doc! {
                    "date": date.clone(),
                    "from": from.clone(),
                    "to": to.clone(),
                    "nft_id": nft_id,
                    "contract": contract.clone(),
                },
                None,
            )
            .await
        {
            Ok(Some(c)) => Some((c, false)),
            _ => {
                let transfer = Transfer {
                    _id: ObjectId::new(),
                    date,
                    from,
                    to,
                    nft: None,
                    nft_id,
                    contract,
                };
                let res = match session {
                    Some(x) => {
                        Transfer::get_collection(client)
                            .insert_one_with_session(transfer.clone(), None, x)
                            .await
                    }
                    _ => {
                        Transfer::get_collection(client)
                            .insert_one(transfer.clone(), None)
                            .await
                    }
                };
                if res.is_err() {
                    println!("transfer {:?}", res.err());
                }
                Some((transfer, true))
            }
        }
    }

    pub async fn save(&self, client: &Client) -> Result<InsertOneResult, Error> {
        Transfer::get_collection(client)
            .insert_one(self, None)
            .await
    }
}
