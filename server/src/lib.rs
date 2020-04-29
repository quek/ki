#[macro_use]
extern crate diesel;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::connection::Connection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use listenfd::ListenFd;
use std::env::var;
use thread_data::ThreadData;

pub mod api;
pub mod auth;
pub mod common;
pub mod errors;
pub mod middleware;
pub mod paginate;
pub mod schema;
pub mod thread_data;
pub mod utils;

// type alias to use in multiple places
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Copy, Clone, Debug)]
pub struct MyConnectionCustomizer;

// diesel が UTC ハードコードしているので
// diesel-1.4.2/src/pg/connection/mod.rs set_config_options
impl<E> r2d2::CustomizeConnection<PgConnection, E> for MyConnectionCustomizer {
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), E> {
        conn.execute("SET TIME ZONE 'Japan'").unwrap();
        Ok(())
    }
}

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "server=debug,actix_web=info,actix_server=info");
    env_logger::init();

    let postgres_url = var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<PgConnection>::new(postgres_url);
    let connection_customizer = Box::new(MyConnectionCustomizer);
    let pool: Pool = r2d2::Pool::builder()
        .connection_customizer(connection_customizer)
        .build(manager)
        .expect("Failed to create pool!");

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
            .data(ThreadData { pool: pool.clone() })
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
