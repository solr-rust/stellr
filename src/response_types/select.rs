//! Utility Structs for deserialising solr responses
//!
//! This module contains a set of structs that you can use to deserialise common solr responses.
//!
//! As the SolrResponseHeader is frequently reused, this is split out into it's own struct, and
//! composed into other types (eg. in SolrSelectType and SolrUpdateType).

use super::SolrResponseHeader;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display};

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
