pub use crate::config::SolrClientConfig;
pub use crate::errors::{SolrError, SolrResult};

#[cfg(not(feature = "blocking"))]
use reqwest::{Client, ClientBuilder, RequestBuilder};

#[cfg(feature = "blocking")]
use reqwest::blocking::{Client, ClientBuilder, RequestBuilder};

use tracing::debug;

/// Add a set of helper methods for "classic Solr" (non-cloud) targets.
///
/// The stellr crate separates out the node access (eg. via DirectSolrClient or ZkSolrClient
/// structs) from the querying of the solr instances (via the SolrCoreMethods and SolrCloudMethods
/// traits). This trait can be used with both Direct and Zookeeper-mediated clients.
///
/// Its main purpose is to create `reqwest::RequestBuilder` instances using the connection
/// information from the underlying client, via the create_get_request and create_post_request
/// methods.
///
/// ```no_run
/// // running solr locally using "solr start -e techproducts"
/// # use tokio::runtime::Runtime;
/// use stellr::prelude::*;
/// use stellr::response_types::SolrSelectType;
/// use serde_json;
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let solr_client = stellr::DirectSolrClient::new("localhost:8983")?;
/// let solr_request = solr_client
///     .create_get_request("/solr/techproducts/select")
///     .expect("HTTP request creation failed...")
///     .q(r#"*:*"#);
///
/// # let mut rt = Runtime::new().unwrap();
/// let solr_result =
/// # rt.block_on(async {
///     solr_request
///     .unstructured_call()
///     .await
///     .expect("Failed to parse")
/// # })
/// ;
///
/// let num_found = solr_result
///     .get("response").unwrap()
///     .get("numFound").unwrap();
///
/// assert_eq!("32", num_found);
/// # Ok(()) }
/// ```
///
/// The one method which _has_ to be implemented is `live_node_url`. All other methods have
/// default implementations.
///
pub trait SolrCoreMethods {
    /// Extract a string-encoded base URL for the protocol and domainname of one host
    fn live_node_url(&self) -> SolrResult<String>;

    /// Return a reference to a SolrClientConfig
    fn request_config(&self) -> &SolrClientConfig;

    /// Return a reference to a SolrClientConfig
    fn set_request_config(&mut self, request_config: SolrClientConfig);

    /// Create a string-encoded URL using a supplied path and the result of live_node_url()
    fn build_request_url(&self, path: &str) -> SolrResult<String> {
        let url: String = format!("{}/{}", self.live_node_url()?, path);
        debug!("Using base url: {}", url);
        Ok(url)
    }

    /// Generate a new reqwest::Client using SolrClientConfig supplied by the caller
    fn build_client(&self) -> SolrResult<Client> {
        let client_builder = ClientBuilder::new();
        let client = client_builder.configure(self.request_config()).build()?;

        Ok(client)
    }

    /// Generate a GET request builder, based on the base_url from the client and a path from the
    /// caller
    fn create_get_request(&self, path: &str) -> SolrResult<RequestBuilder> {
        // let client = self.build_default_client()?;
        let client = self.build_client()?;
        let base_url = self.build_request_url(path)?;
        Ok(client.get(&base_url))
    }

    /// Generate a POST request builder, based on the base_url from the client and a path from the
    /// caller
    fn create_post_request(&self, path: &str) -> SolrResult<RequestBuilder> {
        // let client = self.build_default_client()?;
        let client = self.build_client()?;
        let base_url = self.build_request_url(path)?;
        Ok(client.post(&base_url))
    }
}

/// Adds methods to build an HTTP client configured according to a SolrClientConfig.
///
/// Currently this has only been implemented for the reqwest crate, and applies to
/// reqwuest::ClientBuilder instances. As configure() returns a stock ClientBuilder, you can use
/// the ClientBuilder's own terminus methods to construct the client.
///
/// NB. it is important to use the stellr::prelude to apply this trait to Reqwest::ClientBuilder.
pub trait SolrClientBuilder {
    type HTTPClient;

    /// One-shot client configuration, based on a supplied SolrClientConfig
    fn configure(self, config: &SolrClientConfig) -> Self::HTTPClient;
}

impl SolrClientBuilder for ClientBuilder {
    type HTTPClient = ClientBuilder;

    fn configure(self, config: &SolrClientConfig) -> Self::HTTPClient {
        self.timeout(config.timeout)
            .connection_verbose(config.verbose)
    }
}
