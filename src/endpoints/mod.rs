pub mod contracts;
pub mod health;
pub mod nfts;
pub mod owners;
pub mod transfers;

use std::{env, time::Duration};

use actix_identity::Identity;

use actix_limitation::Limiter;
use actix_session::SessionExt;
use actix_web::{dev::ServiceRequest, web::Data};
pub use health::Health;
use serde::{Deserialize, Serialize};

pub trait Endpoint {
    fn services() -> actix_web::Scope;

    fn default_session_id(req: &ServiceRequest) -> Option<String> {
        let s_id = req
            .connection_info()
            .realip_remote_addr()
            .unwrap_or("unknown")
            .to_string();

        Some(
            req.get_session()
                .get(&s_id)
                .unwrap()
                .unwrap_or(Some(s_id))
                .unwrap()
                + &req.match_pattern().unwrap_or_default(),
        )
    }

    fn limitation() -> Data<Limiter> {
        let redis_url = env::var("REDIS_URL").expect("redis url (REDIS_URL) not set");
        //redis://127.0.0.1:6379
        Data::new(
            Limiter::builder(redis_url)
                .key_by(|req: &ServiceRequest| Self::default_session_id(req))
                .limit(10)
                .period(Duration::from_secs(60)) // 60 minutes
                .build()
                .unwrap(),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    limit: Option<i64>,
    offset: Option<u64>,
}
impl PaginationParams {
    fn limit(&self) -> i64 {
        let l = self.limit.unwrap_or(10);
        if l > 50 {
            return 50;
        }
        l
    }

    fn offset(&self) -> u64 {
        self.offset.unwrap_or_default()
    }
}

pub struct IdentityID {
    #[allow(dead_code)]
    address: String,
    id: String,
}
trait IdentityTrait {
    fn create_id(user_id: String, user_email: String) -> String;
    fn parse_id(&self) -> IdentityID;
}

impl IdentityTrait for Identity {
    fn create_id(user_id: String, user_email: String) -> String {
        format!("{} {}", user_id, user_email)
    }

    fn parse_id(&self) -> IdentityID {
        let binding = self.id().unwrap();
        let sp: Vec<&str> = binding.split(' ').collect();
        IdentityID {
            id: sp[0].to_string(),
            address: sp[1].to_string(),
        }
    }
}
