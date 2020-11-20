#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
//! # strasbourgpark
//!
//!  A crate allowing to interact with the `Strasbourg` for the parkings.
//! The portal hosting the data can be found on `https://data.strasbourg.eu/pages/accueil/`.
//!
//! This provides a ready to use HTTP client fetching the data. You can also used the `api` mod
//!

/// The API objects
pub mod api;
/// The HTTP cleint
mod client;

pub use client::{Client, ClientError};
