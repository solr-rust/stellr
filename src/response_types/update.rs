//! Utility Structs for deserialising solr responses
//!
//! This module contains a set of structs that you can use to deserialise common solr responses.
//!
//! As the SolrResponseHeader is frequently reused, this is split out into it's own struct, and
//! composed into other types (eg. in SolrSelectType and SolrUpdateType).

use super::SolrResponseHeader;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

/// Standard structure for an update response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SolrUpdateType {
    pub responseHeader: SolrResponseHeader,
    pub debug: Option<String>,
}

impl fmt::Display for SolrUpdateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
