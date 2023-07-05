use byc_helpers::web_server::{JsonResponse, Response};

use actix_web::{get, http, web, Responder};
use serde::{Deserialize, Serialize};

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
pub async fn get_health() -> impl Responder {
    (
        Health {
            infos: JsonResponse {
                status: 200,
                message: "ok",
            },
            service_name: "Flow Nft API",
        }
        .to_response(),
        http::StatusCode::OK,
    )
}
