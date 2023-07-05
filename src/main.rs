pub mod endpoints;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    dev::Service,
    http, web, App, HttpServer,
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
    // let secret_key = Key::generate();
    // let redis = RedisSessionStore::new("redis://127.0.0.1:6379")
    //     .await
    //     .unwrap();
    // let domain = "localhost";
    let allowed_origin = "http://localhost:3000";
    let mongo_client = mongo::client::create().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_client.clone()))
            .wrap(get_default_logger_middleware())
            // .wrap(
            //     SessionMiddleware::builder(redis.clone(), secret_key.clone())
            //         .cookie_domain(Some(domain.to_string()))
            //         .cookie_same_site(SameSite::Strict)
            //         .cookie_http_only(false)
            //         .build(),
            // )
            .wrap(
                get_default_cors_middelware()
                    .allowed_origin(allowed_origin)
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().ends_with(b".barteryard.club")
                    })
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
