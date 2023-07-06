use actix::prelude::{Message as ActixMessage, Recipient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(ActixMessage, Deserialize, Serialize, Debug)]
#[rtype(result = "()")]
pub struct MessageToClient {
    pub channel: String,
    pub msg_type: String,
    pub data: Value,
}

impl MessageToClient {
    pub fn new(msg_type: &str, channel: String, data: Value) -> Self {
        Self {
            msg_type: msg_type.to_string(),
            channel,
            data,
        }
    }
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub channel: String,
}

#[derive(ActixMessage)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
