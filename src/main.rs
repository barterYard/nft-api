pub mod endpoints;
use std::{env, time::Duration};

use actix_limitation::{Limiter, RateLimiter};
use actix_session::{storage::RedisSessionStore, SessionExt, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    dev::{Service, ServiceRequest},
    http,
    web::{self, Data},
    App, HttpServer,
};
use byc_helpers::{
    logger::init_logger,
    mongo,
    web_server::{get_default_cors_middelware, get_default_logger_middleware},
};
use endpoints::Health;
use log::info;

use crate::endpoints::{
    contracts::Contracts, nfts::Nfts, owners::Owners, transfers::Transfers, Endpoint,
};

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    init_logger();
    info!("server starting");
    let redis_url = env::var("REDIS_URL").expect("redis url (REDIS_URL) not set"); //redis://127.0.0.1:6379
    let default_limiter = Data::new(
        Limiter::builder(redis_url)
            .key_by(|req: &ServiceRequest| Health::default_session_id(req))
            .limit(1000)
            .period(Duration::from_secs(60)) // 60 minutes
            .build()
            .unwrap(),
    );

    // let domain = "localhost";
    let allowed_origin = "http://localhost:3000";
    let mongo_client = mongo::client::create().await;
    HttpServer::new(move || {
        App::new()
            .wrap(RateLimiter::default())
            .app_data(default_limiter.clone())
            .app_data(web::Data::new(mongo_client.clone()))
            .wrap(get_default_logger_middleware())
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
