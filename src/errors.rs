pub type SolrResult<T> = Result<T, SolrError>;

#[derive(Debug)]
/// SolrError wraps errors from the reqwest and serde crates.
pub enum SolrError {
    /// Catch all error variant
    UnknownSolrError,
    /// Use of not-yet-implemented stellr features
    UnimplementedMethodError,
    /// Reqwest Errors
    HTTPError(reqwest::Error),
    /// Errors from url::parse, mainly from DirectSolrClient
    HostParseError(url::ParseError),
    /// Bad host strings, again mainly from DirectSolrClient
    BadHostError,
    /// Supplied payload does not serialize to a json array
    PayloadNotAJsonArrayError,
    /// Serde errors when trying to deserialise solr responses
    ResponseParseError(serde_json::error::Error),
    /// Zookeeper access errors
    ZookeeperError(zookeeper::ZkError),
}

impl std::error::Error for SolrError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            SolrError::UnknownSolrError => None,
            SolrError::UnimplementedMethodError => None,
            SolrError::HTTPError(_) => None,
            SolrError::HostParseError(_) => None,
            SolrError::BadHostError => None,
            SolrError::PayloadNotAJsonArrayError => None,
            SolrError::ResponseParseError(_) => None,
            SolrError::ZookeeperError(_) => None,
        }
    }
}

impl std::fmt::Display for SolrError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            SolrError::UnknownSolrError => write!(f, "Unclassified Solr Error"),
            SolrError::UnimplementedMethodError => write!(f, "Method Unimplemented"),
            SolrError::HTTPError(ref error) => error.fmt(f),
            SolrError::BadHostError => write!(f, "Could not find a hostname for this client"),
            SolrError::PayloadNotAJsonArrayError => {
                write!(f, "Payload did not deserialize into a JSON array")
            }
            SolrError::HostParseError(ref error) => error.fmt(f),
            SolrError::ResponseParseError(ref error) => error.fmt(f),
            SolrError::ZookeeperError(ref error) => error.fmt(f),
        }
    }
}

impl From<reqwest::Error> for SolrError {
    fn from(err: reqwest::Error) -> SolrError {
        SolrError::HTTPError(err)
    }
}

impl From<url::ParseError> for SolrError {
    fn from(err: url::ParseError) -> SolrError {
        SolrError::HostParseError(err)
    }
}

impl From<serde_json::error::Error> for SolrError {
    fn from(err: serde_json::error::Error) -> SolrError {
        SolrError::ResponseParseError(err)
    }
}

impl From<zookeeper::ZkError> for SolrError {
    fn from(err: zookeeper::ZkError) -> SolrError {
        SolrError::ZookeeperError(err)
    }
}
