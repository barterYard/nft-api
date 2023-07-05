use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use byc_helpers::mongo::{
    models::{common::ModelCollection, mongo_doc, Owner},
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
        web::scope("/owners").service(get_owners)
    }
}

#[get("")]
pub async fn get_owners(
    pagination: web::Query<PaginationParams>,
    client: Data<mongodb::Client>,
) -> impl Responder {
    let owners: Vec<Owner> = match Owner::get_collection(&client)
        .find(
            mongo_doc! {},
            FindOptions::builder()
                .limit(pagination.limit())
                .skip(pagination.offset())
                .allow_partial_results(true)
                .build(),
        )
        .await
    {
        Ok(val) => {
            let t2: Vec<Result<Owner, _>> = val.collect().await;
            t2.into_iter()
                .map(|x| {
                    if x.is_err() {
                        println!("{:?}", x.clone().err())
                    };
                    x.ok().unwrap()
                })
                .collect()
        }
        Err(e) => {
            println!("{:?}", e);
            return (None, http::StatusCode::NOT_FOUND);
        }
    };
    (Some(Json(owners)), http::StatusCode::OK)
}
