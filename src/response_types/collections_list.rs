//! Utility Structs for deserialising solr responses
//!
//! This module contains a set of structs that you can use to deserialise common solr responses.
//!
//! As the SolrResponseHeader is frequently reused, this is split out into it's own struct, and
//! composed into other types (eg. in SolrSelectType and SolrUpdateType).

use super::SolrResponseHeader;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

/// Standard structure for a call to /admin/collections?action=LIST
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SolrCollectionsListType {
    pub responseHeader: SolrResponseHeader,
    pub collections: Vec<String>,
    pub debug: Option<String>,
}

impl fmt::Display for SolrCollectionsListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
