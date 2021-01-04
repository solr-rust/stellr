//! Utility Structs for deserialising solr responses
//!
//! This module contains a set of structs that you can use to deserialise common solr responses.
//!
//! As the SolrResponseHeader is frequently reused, this is split out into it's own struct, and
//! composed into other types (eg. in SolrSelectType and SolrUpdateType).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display};

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

/// Struct to match the standard solr select body
///
/// This does not specify the actual structure of the returned documents, hence the need to
/// parametrise this struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolrSelectBody<T: Debug> {
    pub numFound: u32,
    pub start: u32,
    pub maxScore: Option<f32>,
    pub docs: Vec<T>,
}

impl<T> fmt::Display for SolrSelectBody<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "numFound: {},\nstart: {},\nmaxScore: {:?}\ndocs: {:?}",
            self.numFound, self.start, self.maxScore, self.docs
        )
    }
}

//
// Output Structs for more specialised use-cases
//
/// Standard structure for a full select response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SolrSelectType<T: Debug> {
    pub responseHeader: SolrResponseHeader,
    pub response: SolrSelectBody<T>,
    pub debug: Option<String>,
}

impl<T> fmt::Display for SolrSelectType<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "responseHeader: {}, response: {}, debug: {:?}",
            self.responseHeader, self.response, self.debug
        )
    }
}

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

/// Standard structure for a call to /admin/collections?action=LIST
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct SolrListCollectionsType {
    pub responseHeader: SolrResponseHeader,
    pub collections: Vec<String>,
    pub debug: Option<String>,
}

impl fmt::Display for SolrListCollectionsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
