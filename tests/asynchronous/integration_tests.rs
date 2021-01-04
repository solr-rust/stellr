use serde_json::Value;
use std::panic;
use stellr::prelude::*;
use stellr::response_types::{SolrSelectType, SolrUpdateType};
use stellr::{DirectSolrClient, ZkSolrClient};
use tokio::runtime::Runtime;
use serde::Serialize;

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

        let mut rt = Runtime::new().unwrap();
        let result_struct = rt.block_on(async {
            solr_request
                .call::<SolrSelectType<Value>>()
                .await
                .expect("Failed to parse")
        });

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

        let mut rt = Runtime::new().unwrap();
        let result_struct = rt.block_on(async {
            solr_request
                .unstructured_call()
                .await
                .expect("Failed to parse")
        });

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

    #[allow(non_snake_case)]
    #[derive(Serialize)]
    struct MyData {
        id: String,
        important_number: u32,
    }

    let result = panic::catch_unwind(|| {
        let my_data = vec!(MyData {id: String::from("random-extra-value"), important_number: 42},);
        let solr_client = DirectSolrClient::new("http://localhost:8983/solr").unwrap();
        let solr_request = solr_client
            .update("films")
            .expect("update-client generation failed")
            .payload(&my_data).unwrap()
            .commit();

        let mut rt = Runtime::new().unwrap();
        let result_struct = rt.block_on(async {
            solr_request
                .call::<SolrUpdateType>()
                .await
                .expect("Failed to parse")
        });

        assert_eq!(result_struct.responseHeader.status, 0);
    });

    SOLR_CONTAINER.deregister();
    assert!(result.is_ok());
}
