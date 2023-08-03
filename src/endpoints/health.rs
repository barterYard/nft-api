use flow_helpers::{
    mongo::{
        models::{common::ModelCollection, Contract, GenNft, Owner, Transfer},
        mongodb,
    },
    web_server::{JsonResponse, Response},
};

use actix_web::{
    get, http,
    web::{self, Data, Json},
    Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub infos: JsonResponse,
    pub service_name: &'static str,
}

impl Endpoint for Health {
    fn services() -> actix_web::Scope {
        web::scope("/health").service(get_health)
    }
}
impl Response for Health {
    fn to_response(self) -> actix_web::web::Json<Self> {
        web::Json(self)
    }
}
#[get("")]
pub async fn get_health(client: Data<mongodb::Client>) -> impl Responder {
    let owners = Owner::get_collection(&client)
        .estimated_document_count(None)
        .await
        .unwrap_or_default();
    let transfers = Transfer::get_collection(&client)
        .estimated_document_count(None)
        .await
        .unwrap_or_default();
    let nfts = GenNft::get_collection(&client)
        .estimated_document_count(None)
        .await
        .unwrap_or_default();
    let contracts = Contract::get_collection(&client)
        .estimated_document_count(None)
        .await
        .unwrap_or_default();
    (
        Json(json!({
            "stats": {
                "owners": owners,
                "transfers": transfers,
                "nfts": nfts,
                "contracts": contracts
            },
            "service_name": "NFT Api"
        })),
        http::StatusCode::OK,
    )
}
