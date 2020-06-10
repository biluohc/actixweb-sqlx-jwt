#[macro_use]
extern crate nonblock_logger;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate serde;

use actix_web::{middleware, web, App, HttpServer};

pub mod api;
pub mod config;
pub mod middlewares;
pub mod models;
pub mod state;
pub mod users;

use config::{Config, Opt};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Config::show();
    let (_handle, opt) = Opt::parse_from_args();
    let state = Config::parse_from_file(&opt.config).into_state().await;
    let state2 = state.clone();

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .app_data(state.clone())
            .app_data(web::PathConfig::default().error_handler(api::json_error_handler))
            .app_data(web::JsonConfig::default().error_handler(api::json_error_handler))
            .app_data(web::QueryConfig::default().error_handler(api::json_error_handler))
            .app_data(web::FormConfig::default().error_handler(api::json_error_handler))
            .wrap(middleware::Compress::new(
                actix_web::http::ContentEncoding::Br,
            ))
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(api::notfound))
            .service(web::scope("/user").configure(users::routes::init))
    })
    .bind(&state2.config.listen)?
    .run()
    .await
}
