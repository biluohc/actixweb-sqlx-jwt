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

pub mod api;
pub mod config;
pub mod handlers;
pub mod how;
pub mod middlewares;
pub mod models;
pub mod state;

use config::{Config, Opts};
use salvo::prelude::*;

fn main() {
    use std::sync::atomic::*;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tok-{:02}", id)
        })
        .build()
        .expect("build tokio runtime")
        .block_on(fun())
}

// #[tokio::main]
async fn fun() {
    // Config::show();
    let (_handle, opt) = Opts::parse_from_args();
    let state = Config::parse_from_file(&opt.config).into_state().await;

    let addr = state
        .config
        .listen
        .parse::<std::net::SocketAddr>()
        .map_err(|e| fatal!("parse listenAddr {} failed: {}", state.config.listen, e))
        .unwrap();

    state::global_state_init(state);
    Server::new(handlers::router())
        .with_catchers(middlewares::catcher::defaults::get())
        .try_bind(addr)
        .await
        .map_err(|e| fatal!("listen {} failed: {}", addr, e))
        .unwrap();
}
