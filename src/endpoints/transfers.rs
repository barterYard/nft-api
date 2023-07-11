use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use byc_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, Contract, GenNft, Transfer},
    mongodb::{self, bson::Document, options::FindOptions},
};

use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::endpoints::PaginationParams;

use super::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Transfers {}

impl Endpoint for Transfers {
    fn services() -> actix_web::Scope {
        web::scope("/transfers").service(get_transfers)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFilters {
    contract: Option<String>,
    nft_id: Option<i64>,
    from: Option<String>,
    to: Option<String>,
}

#[get("")]
pub async fn get_transfers(
    pagination: web::Query<PaginationParams>,
    q_filters: web::Query<TransferFilters>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let mut filters = Document::new();
    if q_filters.contract.is_some() {
        if let Ok(Some(contract)) = Contract::get_collection(&client)
            .find_one(
                mongo_doc! {
                    "id": q_filters.0.contract.unwrap()
                },
                None,
            )
            .await
        {
            filters.insert("contract", contract._id);
            println!("{:?}", contract._id);
        };
        if let Some(nft_id) = q_filters.0.nft_id {
            filters.insert("nft_id", nft_id);
        }
    }
    let transfers: Vec<Transfer> = match Transfer::get_collection(&client)
        .find(
            filters,
            FindOptions::builder()
                .limit(pagination.limit())
                .skip(pagination.offset())
                .build(),
        )
        .await
    {
        Ok(val) => {
            let t2: Vec<Result<Transfer, _>> = val.collect().await;
            t2.into_iter().map(|x| x.ok().unwrap()).collect()
        }
        Err(_) => return (None, http::StatusCode::OK),
    };
    (Some(Json(transfers)), http::StatusCode::OK)
}
