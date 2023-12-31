use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use flow_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, Contract},
    mongodb::{self, options::FindOptions},
};

use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::endpoints::PaginationParams;

use super::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Contracts {}

impl Endpoint for Contracts {
    fn services() -> actix_web::Scope {
        web::scope("/contracts").service(get_contracts)
    }
}

#[get("")]
pub async fn get_contracts(
    client: Data<mongodb::Client>,
    pagination: web::Query<PaginationParams>,
) -> impl Responder {
    let contracts: Vec<Contract> = match Contract::get_collection(&client)
        .find(
            mongo_doc! {"done": true},
            FindOptions::builder()
                .limit(pagination.limit())
                .skip(pagination.offset())
                .build(),
        )
        .await
    {
        Ok(val) => {
            let t2: Vec<Result<Contract, _>> = val.collect().await;
            t2.into_iter().map(|x| x.ok().unwrap()).collect()
        }
        Err(_) => return (None, http::StatusCode::OK),
    };
    (Some(Json(contracts)), http::StatusCode::OK)
}
