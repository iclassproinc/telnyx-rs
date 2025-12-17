#![forbid(clippy::unwrap_used)]
#![forbid(clippy::expect_used)]
#![forbid(clippy::panic)]

mod client;
mod error;
pub mod models;
pub mod endpoints;

pub use client::{TelnyxClient, TelnyxClientBuilder};
pub use error::TelnyxError;