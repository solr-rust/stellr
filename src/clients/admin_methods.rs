/*
pub use crate::clients::SolrCoreMethods;
pub use crate::config::SolrClientConfig;
pub use crate::errors::{SolrError, SolrResult};
pub use crate::requests::{SolrRequest, SolrRequestBuilder};

use serde::de::DeserializeOwned;
use crate::response_types::SolrAdminCoresType;

#[cfg(not(feature = "blocking"))]
use reqwest::RequestBuilder;

#[cfg(feature = "blocking")]
use reqwest::blocking::RequestBuilder;

/// Add Solr Admin specific helper methods to any defined SolrClients.
///
/// The stellr crate separates out the node access (eg. via DirectSolrClient or ZkSolrClient
/// structs) from the querying of the solr instances (via the SolrCoreMethods and SolrCloudMethods
/// traits). This trait can be used with both the Direct and Zk-mediated clients.
///
/// The SolrAdminMethods trait builds on the SolrCoreMethods trait to add Solr Admin specific methods,
/// such as to query admin/cores
///
/// ```no_run
/// # use tokio::runtime::Runtime;
/// use stellr::prelude::*;
/// use stellr::response_types::SolrAdminCoresType;
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
///     .call::<SolrAdminCoresType>()
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
pub trait SolrAdminMethods: SolrCoreMethods {
    /// Create a requestBuilder for a solr admin/cores request
    ///
    /// This method assumes the use of a GET method, and does not support POST currently.
    fn admin_cores(&self) -> SolrResult<RequestBuilder> {
        self.create_get_request("admin/cores")
    }
}

#[cfg(not(feature = "blocking"))]
impl RequestBuilder {
    async fn call_admin_cores<T: DeserializeOwned>(self: impl SolrRequest) -> SolrResult<SolrAdminCoresType> {
        self.call::<SolrAdminCoresType>()
    }
}

#[cfg(feature = "blocking")]
impl RequestBuilder {
    fn call_admin_cores<T: DeserializeOwned>(self: impl SolrRequest) -> SolrResult<SolrAdminCoresType> {
        self.call::<SolrAdminCoresType>()
    }
}

*/
