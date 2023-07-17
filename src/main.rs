pub mod endpoints;
use std::{env, time::Duration};

use actix_identity::{Identity, IdentityMiddleware};
use actix_limitation::{Limiter, RateLimiter};
use actix_session::{storage::RedisSessionStore, SessionExt, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    dev::{Service, ServiceRequest},
    http,
    web::{self, Data},
    App, HttpMessage, HttpServer,
};
use byc_helpers::{
    logger::init_logger,
    mongo,
    web_server::{get_default_cors_middelware, get_default_logger_middleware},
};
use endpoints::Health;
use log::info;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::endpoints::{
    contracts::Contracts, nfts::Nfts, owners::Owners, token::Token, transfers::Transfers, Endpoint,
};

#[derive(Debug, Serialize, Deserialize)]
struct LoginParams {
    token: Option<String>,
}

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    init_logger();
    info!("server starting");
    let secret_key = Key::generate();
    let redis_url = env::var("REDIS_URL").expect("redis url (REDIS_URL) not set"); //redis://127.0.0.1:6379
    let default_limiter = Data::new(
        Limiter::builder(redis_url.clone())
            .key_by(|req: &ServiceRequest| {
                let s_id = web::Query::<LoginParams>::from_query(req.query_string())
                    .unwrap()
                    .0;
                let session = req.get_session();
                if let Some(token) = s_id.token {
                    if let Some(token) = session.get::<String>("auth").unwrap() {}

                    return Some(
                        req.get_session()
                            .get(&token)
                            .unwrap()
                            .unwrap_or(Some(token))
                            .unwrap(),
                    );
                } else {
                    let token: String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(16)
                        .map(char::from)
                        .collect();

                    if Identity::login(&req.extensions(), token.clone()).is_ok() {
                        let _ = req.get_session().insert("auth", token);
                    }
                }
                Some("unknown".to_string())
            })
            .limit(100)
            .period(Duration::from_secs(3600)) // 1 hour
            .build()
            .unwrap(),
    );
    let redis = RedisSessionStore::new(redis_url).await.unwrap();

    // let domain = "localhost";
    let allowed_origin = "http://localhost:3000";
    let mongo_client = mongo::client::create().await;
    HttpServer::new(move || {
        App::new()
            .wrap(RateLimiter::default())
            .app_data(default_limiter.clone())
            .app_data(web::Data::new(mongo_client.clone()))
            .wrap(IdentityMiddleware::default())
            .wrap(get_default_logger_middleware())
            .wrap(SessionMiddleware::builder(redis.clone(), secret_key.clone()).build())
            .wrap(
                get_default_cors_middelware()
                    .allowed_origin(allowed_origin)
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials(),
            )
            .service(Health::services())
            .service(Nfts::services())
            .service(Contracts::services())
            .service(Transfers::services())
            .service(Owners::services())
            .service(Token::services())
            .wrap_fn(|req, srv| {
                let fut = srv.call(req);
                async {
                    let mut res = fut.await?;
                    if res.request().method() == "OPTIONS" {
                        res.headers_mut().insert(
                            http::header::ACCESS_CONTROL_ALLOW_HEADERS,
                            http::header::HeaderValue::from_static("content-type"),
                        );
                        res.headers_mut().insert(
                            http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                            http::header::HeaderValue::from_static("true"),
                        );
                    }

                    Ok(res)
                }
            })
    })
    .workers(4)
    .bind(("0.0.0.0", 3005))?
    .run()
    .await
}
