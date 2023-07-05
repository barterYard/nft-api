use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use byc_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, Transfer},
    mongodb::{self, options::FindOptions},
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

#[get("")]
pub async fn get_transfers(
    pagination: web::Query<PaginationParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let contracts: Vec<Transfer> = match Transfer::get_collection(&client)
        .find(
            mongo_doc! {},
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
    (Some(Json(contracts)), http::StatusCode::OK)
}
