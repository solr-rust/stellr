use crate::{SolrError, SolrResult};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Serialize;

#[cfg(not(feature = "blocking"))]
use reqwest::RequestBuilder;

#[cfg(feature = "blocking")]
use reqwest::blocking::RequestBuilder;

/// Helper methods to simplify request building
///
/// This is used as an extension trait to the reqwest::RequestBuilder, and adds methods for common
/// request parameters.
///
/// Raw reqwest::RequestBuilder methods may also be used if a suitable helper method is not found
/// here.
pub trait SolrRequestBuilder {
    fn commit(self) -> RequestBuilder;
    fn content_type(self, content_type: &str) -> RequestBuilder;
    fn debug_query(self, debug: bool) -> RequestBuilder;
    fn fl(self, field_list: &str) -> RequestBuilder;
    fn fq(self, filter_query: &str) -> RequestBuilder;
    fn q(self, query: &str) -> RequestBuilder;
    fn rows(self, row_count: u32) -> RequestBuilder;
    fn wt(self, format: &str) -> RequestBuilder;
    fn payload(self, serializable_payload: &impl Serialize) -> SolrResult<RequestBuilder>;
}

impl SolrRequestBuilder for RequestBuilder {
    /// Mark the query to force a commit
    fn commit(self) -> RequestBuilder {
        self.query(&[("commit", "true")])
    }

    /// Sets Content-Type for this request
    fn content_type(self, content_type: &str) -> RequestBuilder {
        let mut header_map = HeaderMap::new();
        header_map.insert(CONTENT_TYPE, content_type.parse().unwrap());
        self.headers(header_map)
    }

    /// Sets (or unsets) the debugQuery parameter
    fn debug_query(self, debug: bool) -> RequestBuilder {
        if debug {
            self.query(&[("debugQuery", "on")])
        } else {
            self.query(&[("debugQuery", "off")])
        }
    }

    /// limit fields returned (fl) by the request
    fn fl(self, field_list: &str) -> RequestBuilder {
        self.query(&[("fl", field_list)])
    }

    /// Applies a filter query (fq) to the request
    fn fq(self, filter_query: &str) -> RequestBuilder {
        self.query(&[("fq", filter_query)])
    }

    /// Applies a query (q) to the request
    fn q(self, query: &str) -> RequestBuilder {
        self.query(&[("q", query)])
    }

    /// Specifies the number of rows to return
    fn rows(self, row_count: u32) -> RequestBuilder {
        self.query(&[("rows", &row_count.to_string())])
    }

    /// Specifies the response format
    ///
    /// NB. This crate assumes that the output response is in json format, so you should only use
    /// this parameter to set json format on older versions of solr where the default is still XML
    /// (anything pre version 6)
    fn wt(self, format: &str) -> RequestBuilder {
        self.query(&[("wt", format)])
    }

    /// Serialize the supplied list of objects to a json string, and use that for the request body
    ///
    /// The serializeable_payload should be an array or Vec of instances implementing the Serialize
    /// trait.
    ///
    /// NB. Unlike other methods here, this method returns a SolrResult<RequestBuilder>, due to
    /// possible content errors
    fn payload(self, serializable_payload: &impl Serialize) -> SolrResult<RequestBuilder> {
        let json_payload = serde_json::to_string(serializable_payload)?;
        if (&json_payload).starts_with("[") {
            Ok(self.body(json_payload))
        } else {
            Err(SolrError::PayloadNotAJsonArrayError)
        }
    }
}
