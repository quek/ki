use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, web, App, HttpServer};
use listenfd::ListenFd;
use serde::Deserialize;
use std::env::var;
use thread_data::ThreadData;
use tokio_postgres::NoTls;

pub mod api;
pub mod auth;
pub mod common;
pub mod errors;
pub mod generated;
pub mod middleware;
pub mod thread_data;
pub mod utils;

#[derive(Debug, Deserialize)]
struct Config {
    postgres: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("_"))?;
        cfg.try_into()
    }
}

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "server=debug,actix_web=info,actix_server=info");
    env_logger::init();

    let cfg = Config::from_env().unwrap();
    let dpool = cfg.postgres.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .max_age_time(chrono::Duration::hours(12))
                    .secure(false), // this can only be true if you have https
            ))
            .data(ThreadData {
                dpool: dpool.clone(),
            })
            .service(
                web::scope("/api")
                    .route("/hello", web::get().to(api::hello))
                    .route("/login", web::get().to(auth::login))
                    .route("/login-code", web::get().to(auth::login_code))
                    .route("/posts/{id}", web::get().to(api::posts::show))
                    .route("/posts", web::get().to(api::posts::index))
                    .service(
                        web::scope("/admin")
                            .wrap(middleware::EnsureLogin)
                            .route("/posts/{id}/edit", web::get().to(api::admin::posts::edit))
                            .route("/posts/{id}", web::put().to(api::admin::posts::update))
                            .route("/posts", web::get().to(api::admin::posts::index))
                            .route("/posts", web::post().to(api::admin::posts::create)),
                    ),
            )
    });

    let mut listenfd = ListenFd::from_env();
    if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap().run().await
    } else {
        let port = std::env::var("SERVER_PORT").unwrap_or("7778".to_string());
        server.bind(format!("0.0.0.0:{}", port))?.run().await
    }
}
