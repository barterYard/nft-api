use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use byc_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, Contract, GenNft, Owner, Transfer},
    mongodb::{self, bson::Document, options::FindOptions},
};

use super::{Endpoint, PaginationParams};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Nfts {}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetNftParams {
    contract: String,
    nft_id: Option<i64>,
}

impl Endpoint for Nfts {
    fn services() -> actix_web::Scope {
        web::scope("/nfts")
            .service(get_nft)
            .service(get_nfts)
            .service(get_nft_transaction)
    }
}

#[get("/{contract_id}/{nft_id}")]
pub async fn get_nft(
    params: web::Path<(String, i64)>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let (contract_id, nft_id) = params.to_owned();
    let mut filters = Document::new();
    filters.insert("id", nft_id);

    if let Ok(Some(contract)) = Contract::get_collection(&client)
        .find_one(
            mongo_doc! {
                "id": contract_id.clone()
            },
            None,
        )
        .await
    {
        filters.insert("contract", contract._id);
    };
    let nft: GenNft = match GenNft::get_collection(&client)
        .find_one(filters, None)
        .await
    {
        Ok(Some(val)) => val,
        _ => return (None, http::StatusCode::NOT_FOUND),
    };
    let owner = Owner::get_collection(&client)
        .find_one(mongo_doc! {"_id": nft.owner}, None)
        .await
        .unwrap()
        .unwrap_or_default();
    (
        Some(Json(
            json!({"id": nft.id.to_string(), "contract": nft.contract_id, "owner": owner.address }),
        )),
        http::StatusCode::OK,
    )
}

#[get("")]
pub async fn get_nfts(
    query: web::Query<GetNftParams>,
    pagination: web::Query<PaginationParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let mut filters = Document::new();
    if let Some(nft_id) = query.nft_id {
        filters.insert("id", nft_id);
    }
    if let Ok(Some(contract)) = Contract::get_collection(&client)
        .find_one(
            mongo_doc! {
                "id": query.contract.clone()
            },
            None,
        )
        .await
    {
        filters.insert("contract", contract._id);
    };
    let nfts: Vec<GenNft> = match GenNft::get_collection(&client)
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
            let t2: Vec<Result<GenNft, _>> = val.collect().await;
            t2.into_iter().map(|x| x.ok().unwrap()).collect()
        }

        Err(_) => return (None, http::StatusCode::OK),
    };
    (Some(Json(nfts)), http::StatusCode::OK)
}

#[get("/transfers")]
pub async fn get_nft_transaction(
    pagination: web::Query<PaginationParams>,
    query: web::Query<GetNftParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let transferts: Vec<Transfer> = match Transfer::get_collection(&client)
        .find(
            mongo_doc! {"nft_id": query.nft_id, "contract": query.contract.clone()},
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
