pub use crate::clients::SolrCoreMethods;
pub use crate::config::SolrClientConfig;
pub use crate::errors::{SolrError, SolrResult};
pub use crate::requests::SolrRequestBuilder;

#[cfg(not(feature = "blocking"))]
use reqwest::RequestBuilder;

#[cfg(feature = "blocking")]
use reqwest::blocking::RequestBuilder;

/// Add Solr Cloud specific helper methods to any defined SolrClients.
///
/// The stellr crate separates out the node access (eg. via DirectSolrClient or ZkSolrClient
/// structs) from the querying of the solr instances (via the SolrCoreMethods and SolrCloudMethods
/// traits). This trait can be used with both the Direct and Zk-mediated clients.
///
/// The SolrCloudMethods trait builds on the SolrCoreMethods trait to add SolrCloud specific methods,
/// such as to query or update solrcloud collections.
///
/// ```no_run
/// # use tokio::runtime::Runtime;
/// use stellr::prelude::*;
/// use stellr::response_types::SolrSelectType;
/// use serde_json;
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let solr_client = stellr::ZkSolrClient::new("localhost:9983", "/")?;
/// let solr_request = solr_client
///     .select("films")
///     .expect("HTTP request creation failed...")
///     .rows(10)
///     .q(r#"id:"/en/"*"#);
///
/// # let mut rt = Runtime::new().unwrap();
/// let solr_result =
/// # rt.block_on(async {
///     solr_request
///     .call::<SolrSelectType<serde_json::Value>>()
///     .await
///     .expect("Failed to parse")
/// # })
/// ;
///
/// assert_eq!(1100, solr_result.response.numFound);
/// # Ok(()) }
/// ```
///
/// Most of it's methods are based off of the `create_*_request` methods from SolrCoreMethods.
pub trait SolrCloudMethods: SolrCoreMethods {
    /// Create a requestBuilder for a solrCloud select request
    ///
    /// This method assumes the use of a GET method, and does not support POST currently.
    fn select(&self, collection: &str) -> SolrResult<RequestBuilder> {
        self.create_get_request(&format!("{}/{}", collection, "select"))
    }

    /// Create a requestBuilder for a solrCloud update request (using HTTP POST)
    fn update(&self, collection: &str) -> SolrResult<RequestBuilder> {
        self.create_post_request(&format!("{}/{}", collection, "update"))
            .map(|x| x.content_type("application/json"))
    }
}
