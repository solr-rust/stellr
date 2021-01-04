//! # Stellr
//!
//! `Stellr` is a solr client library.
//!
//! It is still under active development, and definitely in an "alpha" state.
//!
//! The crate is heavily dependent on the reqwest and serde crates. The dependence on reqwest means
//! stellr does not support the async-std/smol async runtime, but tokio is supported.
//!
//! The crate supports both non-Cloud and Cloud solr clients, and has non-Cloud and Cloud helper
//! methods for both types of client. Output from requests can be structured with a user-defined
//! type, or unstructured (using serde_json::Value instead).
//!
//! Stellr makes use of extension traits, so you will need to use the prelude for it to work.
//!
//! ## Examples
//!
//! Here is an example of a structured cloud request on a direct client
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use std::error::Error;
//! use stellr::prelude::*;
//! use stellr::DirectSolrClient;
//! use stellr::response_types::SolrSelectType;
//! use serde::Deserialize;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! #[allow(non_snake_case)]
//! #[derive(Debug,Deserialize)]
//! struct MatchingData {
//!     id: String,
//!     popularity: Option<u32>,
//!     inStock: bool,
//! }
//!
//! let solr_client = DirectSolrClient::new("http://localhost:8983")?;
//! let solr_request = solr_client
//!     .select("techproducts")?
//!     .rows(10)
//!     .q("id:SP251*")
//!     .fl("id,popularity,inStock");
//!
//! # let mut rt = Runtime::new().unwrap();
//! let solr_response =
//! # rt.block_on(async {
//!     solr_request
//!     .call::<SolrSelectType<MatchingData>>()
//!     .await
//! # })
//!     ?;
//!
//! assert_eq!(solr_response.response.docs.len(), 1);
//! assert_eq!(solr_response.response.docs[0].popularity, Some(6));
//! # Ok(())
//! # }
//! ```
//!
//! and here's an example of an unstructured request on a cloud client
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use std::error::Error;
//! use stellr::prelude::*;
//! use stellr::ZkSolrClient;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let solr_client = ZkSolrClient::new("localhost:9983", "/")?;
//! let solr_request = solr_client
//!     .select("gettingstarted")?
//!     .rows(10)
//!     .q("*:*");
//!
//! # let mut rt = Runtime::new().unwrap();
//! let result_struct =
//! # rt.block_on(async {
//!     solr_request
//!     .unstructured_call()
//!     .await
//! # })
//!     ;
//!
//! let docs_count = result_struct?
//!     .get("response").unwrap()
//!     .get("docs").unwrap()
//!     .as_array().unwrap()
//!     .len();
//!
//! assert_eq!(10, docs_count);
//! # Ok(())
//! # }
//! ```
//!
//! Finally here is an example of data insertion
//!
//! ```no_run
//! # use tokio::runtime::Runtime;
//! # use std::error::Error;
//! use stellr::prelude::*;
//! use stellr::DirectSolrClient;
//! use stellr::response_types::SolrUpdateType;
//! use serde::Serialize;
//!
//! #[allow(non_snake_case)]
//! #[derive(Serialize)]
//! struct MyData {
//!     id: String,
//!     important_number: u32,
//! }
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # let mut rt = Runtime::new().unwrap();
//! let solr_client = DirectSolrClient::new("http://localhost:8983")?;
//! let my_data = vec!(MyData {id: String::from("random-extra-value"), important_number: 42},);
//! let solr_request = solr_client
//!     .update("films")?
//!     .payload(&my_data)?
//!     .commit();
//!
//! let result_struct =
//! # rt.block_on(async {
//!     solr_request
//!         .call::<SolrUpdateType>()
//!         .await
//! # });
//!
//! assert_eq!(result_struct?.responseHeader.status, 0);
//! # Ok(()) }
//! ```
//!
//! ## Package Features
//!
//! The crate only has one optional feature:
//!
//! * `blocking` - use the blocking feature of the reqwest library, rather than the async version
//!
//! ## Errors
//!
//! Stellr defines a `SolrError` struct, which wraps errors types from the serde, reqwest and
//! zookeeper crates.
//!
//! ## Safety
//!
//! This crate introduces no new unsafe code, beyond what already exists in it's dependencies.

#![allow(non_snake_case)]

#[doc(inline)]
pub use clients::{
    DirectSolrClient, SolrClientBuilder, SolrCloudMethods, SolrCoreMethods, ZkSolrClient,
};
pub use config::SolrClientConfig;
pub use errors::{SolrError, SolrResult};
#[doc(inline)]
pub use requests::{SolrRequest, SolrRequestBuilder};

mod clients;
mod config;
mod errors;
pub mod prelude;
mod requests;
pub mod response_types;
