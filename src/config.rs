use std::fmt;
use std::time::Duration;

/// Configuration object for SolrClients
///
/// This struct stores request configuration information, which will be applied to every request
/// for a solr client.
///
/// ```
/// use stellr::prelude::*;
/// use stellr::SolrClientConfig;
///
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let solr_config = SolrClientConfig { verbose: true, ..Default::default() };
/// let solr_client = stellr::DirectSolrClient::new("http://localhost:8983")?
///     .set_request_config(solr_config);
/// # Ok(()) }
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SolrClientConfig {
    /// HTTP request timeout
    pub timeout: Duration,
    /// Be verbose when making HTTP requests
    pub verbose: bool,
}

impl SolrClientConfig {
    pub fn new() -> SolrClientConfig {
        SolrClientConfig {
            verbose: false,
            timeout: Duration::from_secs(20),
        }
    }

    /// Set a custom timeout
    pub fn timeout(self, timeout: Duration) -> SolrClientConfig {
        SolrClientConfig { timeout, ..self }
    }

    /// Set the client to be more verbose
    pub fn verbose(self, verbose: bool) -> SolrClientConfig {
        SolrClientConfig { verbose, ..self }
    }
}

impl Default for SolrClientConfig {
    fn default() -> Self {
        SolrClientConfig {
            verbose: false,
            timeout: Duration::from_secs(20),
        }
    }
}

impl fmt::Display for SolrClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SolrClientConfig(timeout: {:?}, verbose: {})",
            self.timeout, self.verbose
        )
    }
}
