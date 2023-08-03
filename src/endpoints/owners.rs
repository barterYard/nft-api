use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use flow_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, GenNft, Owner},
    mongodb::{self, options::FindOptions},
};

use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::endpoints::PaginationParams;

use super::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Owners {}

impl Endpoint for Owners {
    fn services() -> actix_web::Scope {
        web::scope("/owners")
            .service(get_owner)
            .service(get_owner_nfts)
    }
}

#[get("/{address}")]
pub async fn get_owner(
    address: web::Path<String>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let owner: Owner = match Owner::get_collection(&client)
        .find_one(mongo_doc! {"address": address.to_string()}, None)
        .await
    {
        Ok(Some(val)) => val,
        _ => {
            return (None, http::StatusCode::NOT_FOUND);
        }
    };
    (Some(Json(owner)), http::StatusCode::OK)
}

#[get("/{address}/nfts")]
pub async fn get_owner_nfts(
    address: web::Path<String>,
    pagination: web::Query<PaginationParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let owner: Owner = match Owner::get_collection(&client)
        .find_one(mongo_doc! {"address": address.to_string()}, None)
        .await
    {
        Ok(Some(val)) => val,
        _ => {
            return (None, http::StatusCode::NOT_FOUND);
        }
    };
    let nfts: Vec<GenNft> = match GenNft::get_collection(&client)
        .find(
            mongo_doc! {"owner": owner._id},
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
        Err(_) => return (None, http::StatusCode::NOT_FOUND),
    };
    (Some(Json(nfts)), http::StatusCode::OK)
}
