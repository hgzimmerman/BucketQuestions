//! A library that contains all of the sql interfacing logic the server uses.

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

#[macro_use]
extern crate diesel;

//pub mod adaptive_health;
//pub mod event;
mod schema;
//pub mod stock;
pub mod user;
mod util;
