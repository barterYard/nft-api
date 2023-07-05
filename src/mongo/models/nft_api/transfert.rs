use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;
use log::{error, info};
use mongodb::{error::Error, results::InsertOneResult, Client};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Transfert {
    pub _id: ObjectId,
    pub date: String,
    pub from: String,
    pub to: String,
    pub nft: ObjectId,
}

impl Transfert {
    pub fn new(date: String, from: String, to: String, nft: ObjectId) -> Self {
        Transfert {
            _id: ObjectId::new(),
            date,
            from,
            to,
            nft,
        }
    }

    pub async fn create(
        date: String,
        from: String,
        to: String,
        nft: ObjectId,
        client: &Client,
    ) -> Result<InsertOneResult, Error> {
        let transfer = Transfert {
            _id: ObjectId::new(),
            date,
            from,
            to,
            nft,
        };
        Transfert::get_collection(client)
            .insert_one(transfer, None)
            .await
    }

    pub async fn get_or_create(
        date: String,
        from: String,
        to: String,
        nft: ObjectId,
        client: &Client,
    ) -> Option<(Transfert, bool)> {
        let transfer_col = Transfert::get_collection(client);

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
            Ok(Some(c)) => Some((c, false)),
            _ => {
                let transfer = Transfert {
                    _id: ObjectId::new(),
                    date,
                    from,
                    to,
                    nft,
                };
                match Transfert::get_collection(client)
                    .insert_one(transfer.clone(), None)
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        error!("{}", e);
                    }
                };

                Some((transfer, true))
            }
        }
    }

    pub async fn save(&self, client: &Client) -> Result<InsertOneResult, Error> {
        Transfert::get_collection(client)
            .insert_one(self, None)
            .await
    }
}
