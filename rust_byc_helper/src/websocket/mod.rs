use actix::{
    fut,
    prelude::{Actor, Addr, Handler},
    ActorContext, ActorFutureExt, AsyncContext, ContextFutureSpawner, StreamHandler, WrapFuture,
};
use actix_web::{
    get,
    web::{self},
    Error, HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use log::{info, warn};
use std::time::{Duration, Instant};

pub mod messages;
pub mod server;

pub use self::server::*;

use self::messages::{Connect, Disconnect, Message};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(30);

pub struct WebSocketSession {
    id: usize,
    hb: Instant,
    channel: String,
    server_addr: Addr<Server>,
}

impl WebSocketSession {
    fn new(server_addr: Addr<Server>, channel: String) -> Self {
        Self {
            id: 0,
            hb: Instant::now(),
            channel,
            server_addr,
        }
    }

    fn send_heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                info!("Websocket Client heartbeat failed, disconnecting!");
                act.server_addr.do_send(Disconnect { id: act.id.clone() });
                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.send_heartbeat(ctx);

        let session_addr = ctx.address();

        self.server_addr
            .send(Connect {
                addr: session_addr.recipient(),
                channel: self.channel.clone(),
            })
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(_res) => {}
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                self.send_heartbeat(ctx);
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                self.server_addr.do_send(Disconnect {
                    id: self.id.clone(),
                });
                ctx.close(reason);
                ctx.stop();
            }
            Err(err) => {
                warn!("Error handling msg: {:?}", err);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<Message> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub fn services() -> actix_web::Scope {
    web::scope("/ws").service(ws_index)
}

#[get("/{channel}")]
pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    channel: web::Path<String>,
    server_addr: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    let res = ws::start(
        WebSocketSession::new(server_addr.get_ref().clone(), channel.into_inner()),
        &req,
        stream,
    )?;

    Ok(res)
}
