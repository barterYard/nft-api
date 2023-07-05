use std::collections::{HashMap, HashSet};

use actix::{prelude::Recipient, Actor, Context, Handler};
use log::error;

use rand::{self, rngs::ThreadRng, Rng};
use serde_json::{error::Result as SerdeResult, to_string};

use super::messages::{Connect, Disconnect, Message, MessageToClient};

#[derive(Default)]
pub struct Server {
    pub sessions: HashMap<usize, Recipient<Message>>,
    pub channels: HashMap<String, HashSet<usize>>,
    pub rng: ThreadRng,
}

impl Server {
    pub fn new() -> Self {
        Server {
            sessions: HashMap::new(),
            channels: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }
    pub fn send_message(&self, data: SerdeResult<String>, channel: String) {
        match data {
            Ok(data) => {
                if let Some(sessions) = self.channels.get(&channel) {
                    for id in sessions {
                        if let Some(addr) = self.sessions.get(id) {
                            addr.do_send(Message(data.to_owned()));
                        }
                    }
                }
            }
            Err(err) => {
                error!("Data did not convert to string {:?}", err);
            }
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        let id = self.rng.gen::<usize>();

        // auto join session to main room
        self.sessions.insert(id, msg.addr);
        self.channels
            .entry(msg.channel.to_owned())
            .or_insert_with(HashSet::new)
            .insert(id);
    }
}

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<MessageToClient> for Server {
    type Result = ();

    fn handle(&mut self, msg: MessageToClient, _: &mut Context<Self>) -> Self::Result {
        self.send_message(to_string(&msg), msg.channel);
    }
}
