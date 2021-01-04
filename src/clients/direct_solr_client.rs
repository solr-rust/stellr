#![allow(non_snake_case)]

use super::cloud_methods::SolrCloudMethods;
use crate::config::SolrClientConfig;
use crate::{SolrCoreMethods, SolrError, SolrResult};
use std::fmt;
use url::Url;

/// Directly access a solr node via http.
///
/// This takes the name of the target as an argument to new(), either as a URL
///
/// ```
/// use stellr::prelude::*;
/// use stellr::DirectSolrClient;
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let my_client = DirectSolrClient::new("http://localhost:8983/solr")?;
/// assert_eq!(my_client.live_node_url().unwrap(), "http://localhost:8983/solr");
/// # Ok(()) }
/// ```
///
/// or as a node_name
///
/// ```
/// use stellr::prelude::*;
/// use stellr::DirectSolrClient;
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let my_client = DirectSolrClient::new("10.23.45.6:8080_solr")?;
/// assert_eq!(my_client.live_node_url().unwrap(), "http://10.23.45.6:8080/solr");
/// # Ok(()) }
/// ```
///
/// Please see the SolrCoreMethods and SolrCloudMethods Traits for details on how to use these
/// clients.
///
/// ## Errors
///
/// The new() method can create BadHostErrors and HostParseErrors if the supplied name is bad.
///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DirectSolrClient {
    host: String,
    port: u16,
    context: String,
    request_config: SolrClientConfig,
}

impl DirectSolrClient {
    /// Assemble a new client from a hostname and port.
    ///
    /// This method assumes that the context is /solr, if not please use `new_with_context`
    pub fn new(host_address: &str) -> SolrResult<DirectSolrClient> {
        let (host, port, context) = split_host_address(host_address)?;

        Ok(DirectSolrClient {
            host,
            port,
            context,
            request_config: SolrClientConfig::new(),
        })
    }
}

/// Separate out host, port, context from a supplied URL or solr node name
///
/// This will need to cope with the following variants
///  * http://localhost/solr
///  * http://localhost:8983
///  * localhost:8080
///  * 127.0.0.1:8080_solr
fn split_host_address(host_address: &str) -> SolrResult<(String, u16, String)> {
    if host_address.starts_with("http") {
        split_url(host_address)
    } else {
        split_node_name(host_address)
    }
}

/// Split apart a url into a (host, port, context) tuple
///
/// This function will assume port 80 if no port is specified, and will return a
/// BadHostError if the domainname cannot be extracted
fn split_url(url: &str) -> SolrResult<(String, u16, String)> {
    let url_details = Url::parse(url)?;

    if url_details.host_str().is_none() {
        return Err(SolrError::BadHostError);
    }
    let host = url_details.host_str().unwrap().to_string();
    let port = url_details.port().unwrap_or(80);
    let context = url_details.path().to_string();

    Ok((host, port, context))
}

/// Split apart a solr node name into a (host, port, context) tuple
///
/// This assumes the name will be of the form host:port_context (eg. 127.0.0.1:8080_solr)
fn split_node_name(node_name: &str) -> SolrResult<(String, u16, String)> {
    let split_by_port: Vec<_> = node_name.split(':').collect();
    if split_by_port.len() != 2 {
        // multiple colons means we don't know where the host/port split is
        return Err(SolrError::BadHostError);
    }
    let host = split_by_port[0].to_string();

    let split_by_context: Vec<_> = split_by_port[1].split('_').collect();
    let port: u16 = match split_by_context[0].parse() {
        Ok(v) => v,
        Err(_) => return Err(SolrError::BadHostError),
    };

    let mut context = "/".to_string();
    if split_by_context.len() > 1 {
        context.push_str(&split_by_context[1..].join("/"))
    }

    Ok((host, port, context))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solr_node() {
        let test_node = "127.0.0.1:8080_solr";
        let result_tuple = split_node_name(test_node).unwrap();
        assert_eq!(
            result_tuple,
            ("127.0.0.1".to_string(), 8080u16, "/solr".to_string())
        );
    }
}

impl SolrCoreMethods for DirectSolrClient {
    fn live_node_url(&self) -> SolrResult<String> {
        Ok(format!(
            "http://{}:{}{}",
            self.host, self.port, self.context
        ))
    }

    fn request_config(&self) -> &SolrClientConfig {
        &self.request_config
    }

    fn set_request_config(&mut self, request_config: SolrClientConfig) {
        self.request_config = request_config;
    }
}

/// Support the SolrCloud methods for a direct connection too
impl SolrCloudMethods for DirectSolrClient {}

impl Default for DirectSolrClient {
    fn default() -> Self {
        DirectSolrClient {
            host: String::from("localhost"),
            port: 8983,
            context: String::from("solr"),
            request_config: SolrClientConfig::new(),
        }
    }
}

impl fmt::Display for DirectSolrClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DirectSolrClient({}:{}_{})",
            self.host, self.port, self.context
        )
    }
}
