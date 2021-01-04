pub use crate::requests::{SolrRequest, SolrRequestBuilder};
/// Stellr prelude
///
/// We need to add a prelude to catch the extension traits (Eg. SolrCloudMethods and
/// SolrRequestBuilder)
pub use crate::{SolrClientBuilder, SolrCloudMethods, SolrCoreMethods};
pub use crate::{SolrError, SolrResult};
