#[macro_use]
extern crate nonblock_logger;
#[macro_use]
extern crate async_trait;
// #[macro_use]
// extern crate anyhow;
#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate serde;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};

pub mod api;
pub mod config;
pub mod how;
pub mod middlewares;
pub mod models;
pub mod state;
pub mod users;

use config::{Config, Opts};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Config::show();
    let (_handle, opt) = Opts::parse_from_args();
    let state = Config::parse_from_file(&opt.config).into_state().await;
    let state2 = state.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(state.clone())
            .app_data(web::PathConfig::default().error_handler(api::json_error_handler))
            .app_data(web::JsonConfig::default().error_handler(api::json_error_handler))
            .app_data(web::QueryConfig::default().error_handler(api::json_error_handler))
            .app_data(web::FormConfig::default().error_handler(api::json_error_handler))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(api::notfound))
            .service(web::scope("/user").configure(users::routes::init))
            .service(
                Files::new("/assets", "assets")
                    .redirect_to_slash_directory()
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .keep_alive(300)
    .bind(&state2.config.listen)?
    .run()
    .await
}
