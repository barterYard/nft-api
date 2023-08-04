use actix_identity::Identity;
use actix_session::Session;
use actix_web::{
    get, http,
    web::{self, Json},
    HttpMessage, HttpRequest, Responder,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::Endpoint;

#[derive(Serialize, Deserialize)]
pub struct Token {}

impl Endpoint for Token {
    fn services() -> actix_web::Scope {
        web::scope("/token").service(get_token)
    }
}

#[get("")]
pub async fn get_token(req: HttpRequest, session: Session) -> impl Responder {
    if let Some(token) = session.get::<String>("auth").unwrap() {
        return (Some(Json(json!({ "token": token }))), http::StatusCode::OK);
    }
    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    if Identity::login(&req.extensions(), token.clone()).is_ok() {
        let _ = session.insert("auth", token.clone());
    }
    (Some(Json(json!({ "token": token }))), http::StatusCode::OK)
}
