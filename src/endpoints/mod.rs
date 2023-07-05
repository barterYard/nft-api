pub mod contracts;
pub mod health;
pub mod nfts;
pub mod owners;
pub mod transfers;

use actix_identity::Identity;

pub use health::Health;
use serde::{Deserialize, Serialize};

pub trait Endpoint {
    fn services() -> actix_web::Scope;
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
