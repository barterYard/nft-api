use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use byc_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, GenNft, Transfer},
    mongodb::{self, options::FindOptions},
};

use futures::StreamExt;
use serde::{Deserialize, Serialize};

use super::{Endpoint, PaginationParams};

#[derive(Serialize, Deserialize)]
pub struct Nfts {}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetNftParams {
    contract: String,
    nft_id: String,
}

impl Endpoint for Nfts {
    fn services() -> actix_web::Scope {
        web::scope("/nfts")
            .service(get_nft)
            .service(get_nft_transaction)
    }
}

#[get("")]
pub async fn get_nft(
    query: web::Query<GetNftParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let nft = match GenNft::get_collection(&client)
        .find_one(
            mongo_doc! {"contract_id": query.contract.clone(), "id": query.nft_id.clone()},
            None,
        )
        .await
    {
        Ok(Some(val)) => val,
        Ok(None) => return (None, http::StatusCode::OK),
        Err(_) => return (None, http::StatusCode::OK),
    };
    (Some(Json(nft)), http::StatusCode::OK)
}

#[get("/transfers")]
pub async fn get_nft_transaction(
    pagination: web::Query<PaginationParams>,
    query: web::Query<GetNftParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let nft = match GenNft::get_collection(&client)
        .find_one(
            mongo_doc! {"contract_id": query.contract.clone(), "id": query.nft_id.clone()},
            None,
        )
        .await
    {
        Ok(Some(val)) => val,
        Ok(None) => return (None, http::StatusCode::OK),
        Err(_) => return (None, http::StatusCode::OK),
    };
    let transferts: Vec<Transfer> = match Transfer::get_collection(&client)
        .find(
            mongo_doc! {"nft": nft._id},
            FindOptions::builder()
                .limit(pagination.limit())
                .skip(pagination.offset())
                .build(),
        )
        .await
    {
        Ok(t) => {
            let t2: Vec<Result<Transfer, _>> = t.collect().await;
            t2.into_iter().map(|x| x.ok().unwrap()).collect()
        }
        _ => return (None, http::StatusCode::OK),
    };

    (Some(Json(transferts)), http::StatusCode::OK)
}
