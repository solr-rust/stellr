//! Utility Structs for deserialising solr responses
//!
//! This module contains a set of structs that you can use to deserialise common solr responses.
//!
//! As the SolrResponseHeader is frequently reused, this is split out into it's own struct, and
//! composed into other types (eg. in SolrSelectType and SolrUpdateType).

mod admin_cores;
mod collections_list;
mod select;
mod update;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug, Display};

pub use admin_cores::*;
pub use collections_list::*;
pub use select::*;
pub use update::*;

//
// Generic/Common Structs
//
/// Struct to match the standard solr responseHeader
///
/// This is typically used as part of SolrRequest::call<T>() as part of a more complex response
/// type.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SolrResponseHeader {
    pub status: u32,
    pub QTime: u32,
    pub params: Option<HashMap<String, String>>,
    pub rf: Option<u32>,
    pub zkConnected: Option<bool>,
}

impl fmt::Display for SolrResponseHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
