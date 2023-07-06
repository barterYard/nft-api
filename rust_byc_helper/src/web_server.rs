use crate::logger::get_log_format;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};

pub trait Endpoint {
    fn services() -> actix_web::Scope;
}

pub trait IdentityID {
    fn create_id(user_id: String, user_email: String, token: Option<String>) -> String {
        format!("{} {} {}", user_id, user_email, token.unwrap_or_default())
    }
    fn parse_id(&self) -> (String, String, Option<String>);
}

#[derive(Serialize, Deserialize)]
pub struct JsonResponse {
    pub status: i16,
    pub message: &'static str,
}

pub trait Response {
    fn to_response(self) -> actix_web::web::Json<Self>
    where
        Self: std::marker::Sized;
}

pub fn get_default_cors_middelware() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "HEAD", "OPTIONS"])
        .max_age(144000)
}

pub fn get_default_logger_middleware() -> actix_web::middleware::Logger {
    Logger::new(get_log_format())
}
