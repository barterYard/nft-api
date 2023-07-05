use crate::mongo::models::{common::ModelCollection, mongo_doc};

use mongodb::Client;
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct User {
    pub _id: bson::oid::ObjectId,
    pub address: String,
    pub email: String,
    pub username: Option<String>,
    pub parent_address: Option<String>,
}

impl User {
    pub async fn get_by_email(client: &Client, email: &str) -> Option<User> {
        let users_db = User::get_collection(client);
        if let Ok(Some(user)) = users_db.find_one(mongo_doc! {"email": email}, None).await {
            Some(user)
        } else {
            None
        }
    }

    pub async fn update(
        self,
        client: &Client,
        username: String,
    ) -> Result<User, mongodb::error::Error> {
        let users_db = User::get_collection(client);
        match users_db
            .update_one(
                mongo_doc! {
                    "email": self.email.clone()
                },
                mongo_doc! {
                    "username": username.clone()
                },
                None,
            )
            .await
        {
            Ok(_) => {
                let n_user = User {
                    username: Some(username),
                    ..self.clone()
                };
                Ok(n_user)
            }
            Err(e) => Err(e),
        }
    }
}
