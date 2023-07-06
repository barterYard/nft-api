use std::str::FromStr;

use crate::mongo::models::{common::ModelCollection, mongo_doc, Werewolf};
use bson::{oid::ObjectId, Bson, DateTime};
use futures::TryStreamExt;
use mongodb::{results::UpdateResult, Client};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReducedUser {
    pub name: String,
    pub address: String,
}

impl From<ReducedUser> for Bson {
    fn from(value: ReducedUser) -> Self {
        Bson::Document(mongo_doc! {
            "name": value.name,
            "address": value.address

        })
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "senderID")]
    pub sender_id: ObjectId,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,
    #[serde(rename = "readAt")]
    pub read_at: Option<DateTime>,

    pub sender: Option<ReducedUser>,
    pub text: String,
}

impl Message {
    pub fn as_value(&self) -> serde_json::Value {
        json!({
            "_id": self.id.to_string(),
            "createdAt": self.created_at.timestamp_millis(),
            "senderID": self.sender_id.to_string(),
            "sender": self.sender,
            "text": self.text.clone(),
            "readAt": self.read_at,
        })
    }
    pub fn new(sender: Werewolf, text: String) -> Self {
        return Message {
            id: ObjectId::new(),
            created_at: DateTime::now(),
            sender_id: sender._id,
            text,
            read_at: None,
            sender: Some(ReducedUser {
                name: sender.address.clone(),
                address: sender.address,
            }),
        };
    }
}

impl From<Message> for Bson {
    fn from(value: Message) -> Self {
        Bson::Document(mongo_doc! {
            "_id": value.id,
            "createdAt": value.created_at,
            "senderID": value.sender_id,
            "sender": value.sender,
            "text": value.text.clone(),
            "readAt": value.read_at,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ModelCollection)]
pub struct Conversation {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    #[serde(rename = "barterID")]
    pub barter_id: i64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime,

    pub name: String,
    pub members: Vec<ObjectId>,
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn as_value(&self) -> serde_json::Value {
        json!({
            "_id": self.id.to_string(),
            "barter_id": self.barter_id,
            "members": self.members.iter().map(|u| u.to_string()).collect::<Vec<String>>(),
            "name": self.name,
            "messages": self.messages.iter().map(|m| m.as_value()).collect::<Vec<serde_json::Value>>(),
            "created_at": self.created_at.timestamp_millis(),
        })
    }

    pub fn new(barter_id: i64, name: String) -> Conversation {
        Conversation {
            barter_id: barter_id,
            id: ObjectId::new(),
            members: vec![],
            name,
            messages: vec![],
            created_at: DateTime::now(),
        }
    }

    pub async fn get_or_create_conversation_by_barter_id(
        client: &Client,
        barter_id: i64,
    ) -> Option<Conversation> {
        match Conversation::get_conversation_by_barter_id(client, barter_id).await {
            Some(y) => Some(y),
            None => {
                let conv = Conversation::get_collection(client);
                let new_conv = Conversation::new(barter_id, barter_id.to_string().clone());
                let _ = conv.insert_one(&new_conv, None).await;
                Some(new_conv)
            }
        }
    }

    pub async fn get_conversations_for_user(user_id: String, client: &Client) -> Vec<Conversation> {
        let conv = Conversation::get_collection(&client);
        match conv
            .find(
                mongo_doc! {
                    "members": ObjectId::from_str(&user_id).unwrap()
                },
                None,
            )
            .await
        {
            Ok(x) => x.try_collect().await.unwrap_or_default(),
            _ => vec![],
        }
    }

    pub async fn get_conversation_by_barter_id(
        client: &Client,
        barter_id: i64,
    ) -> Option<Conversation> {
        let conv = Conversation::get_collection(client);

        match conv
            .find_one(mongo_doc! {"barterID": barter_id}, None)
            .await
        {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }

    pub async fn add_message(
        &mut self,
        client: &Client,
        message: Message,
    ) -> Result<UpdateResult, mongodb::error::Error> {
        let conv = Conversation::get_collection(client);
        self.messages.push(message.clone());

        conv.update_one(
            mongo_doc! {
                "_id": self.id
            },
            mongo_doc! {
                "$push" : {
                    "messages": message.clone(),

                },
                "$addToSet" : {
                    "members": message.clone().sender_id
                }
            },
            None,
        )
        .await
    }
}
