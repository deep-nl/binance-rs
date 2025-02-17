#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces,
    clippy::all
)]
#![allow(clippy::needless_doctest_main)]
#![warn(
    clippy::wildcard_imports,
    clippy::manual_string_new,
    clippy::single_match_else,
    clippy::implicit_clone,
    clippy::semicolon_if_nothing_returned
)]

mod client;
mod clientV2;
pub mod errors;
pub mod util;

pub mod model;

pub mod account;
pub mod accountV2;
pub mod api;
pub mod apiV2;
pub mod config;
pub mod general;
pub mod market;
pub mod savings;
pub mod userstream;
pub mod userstreamV2;
pub mod websockets;

pub mod futures;
