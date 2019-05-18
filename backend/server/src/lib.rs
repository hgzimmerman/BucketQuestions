//! Crate that defines the http routes and the business logic.
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

mod api;
mod config;
mod error;
mod server_auth;
mod state;
mod static_files;
mod util;

pub use config::Config;

use crate::{
    api::routes,
    server_auth::get_google_login_link,
    state::{state_config::StateConfig, State},
};
use log::info;

/// Starts the server.
pub fn start(config: Config) {
    info!("{:#?}", config);
    let localhost = [0, 0, 0, 0];
    let addr = (localhost, config.port);

    let state_config = StateConfig::from(config.clone());
    let state = State::new(state_config);
    info!("{:#?}", state);

    let routes = routes(&state);

    if config.tls_enabled {
        warp::serve(routes)
            .tls("tls/cert.pem", "tls/key.rsa")
            .run(addr);
    } else {
        warp::serve(routes).run(addr);
    }
}
