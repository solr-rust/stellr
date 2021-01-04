use serde_json::Value;
use std::panic;
use stellr::prelude::*;
use stellr::response_types::{SolrSelectType, SolrUpdateType};
use stellr::{DirectSolrClient, ZkSolrClient};

use super::it::SolrContainer;

lazy_static! {
    static ref SOLR_CONTAINER: SolrContainer = SolrContainer::new();
}

#[test]
#[ignore]
fn structured_select_test() {
    SOLR_CONTAINER.register();

    let result = panic::catch_unwind(|| {
        let solr_client = ZkSolrClient::new("localhost:9983", "/").unwrap();
        let solr_request = solr_client
            .select("films")
            .unwrap()
            .rows(10)
            .q(r#"id:"/en/"*"#);

        let result_struct = solr_request
            .call::<SolrSelectType<Value>>()
            .expect("Failed to parse");

        assert_eq!(1100, result_struct.response.numFound);
    });

    SOLR_CONTAINER.deregister();
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn unstructured_select_test() {
    SOLR_CONTAINER.register();

    let result = panic::catch_unwind(|| {
        let solr_client = DirectSolrClient::new("http://localhost:8983/solr").unwrap();
        let solr_request = solr_client.select("films").unwrap().rows(10).q("*:*");

        let result_struct = solr_request.unstructured_call().expect("Failed to parse");

        let docs_count = result_struct
            .get("response")
            .unwrap()
            .get("docs")
            .unwrap()
            .as_array()
            .unwrap()
            .len();
        assert_eq!(10, docs_count);
    });

    SOLR_CONTAINER.deregister();
    assert!(result.is_ok());
}

#[test]
#[ignore]
fn structured_insertion_test() {
    SOLR_CONTAINER.register();

    let result = panic::catch_unwind(|| {
        let solr_client = DirectSolrClient::new("http://localhost:8983/solr").unwrap();
        let solr_request = solr_client
            .update("films")
            .expect("update-client generation failed")
            .body(r#"[{"id":"random-extra-value", "extra-key":"extra-value"}]"#)
            .commit();

        let result_struct = solr_request
            .call::<SolrUpdateType>()
            .expect("Failed to parse");

        assert_eq!(result_struct.responseHeader.status, 0);
    });

    SOLR_CONTAINER.deregister();
    assert!(result.is_ok());
}
