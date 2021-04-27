pub mod admin_methods;
pub mod cloud_methods;
pub mod core_methods;
pub mod direct_solr_client;
pub mod zk_solr_client;

/*
#[doc(inline)]
pub use admin_methods::SolrAdminMethods;
*/
#[doc(inline)]
pub use cloud_methods::SolrCloudMethods;
#[doc(inline)]
pub use core_methods::{SolrClientBuilder, SolrCoreMethods};
#[doc(inline)]
pub use direct_solr_client::DirectSolrClient;
#[doc(inline)]
pub use zk_solr_client::ZkSolrClient;
