use serde::Deserialize;

use stellr::prelude::*;
use stellr::response_types::SolrSelectType;
use stellr::DirectSolrClient;

use tokio::runtime::Runtime;

#[test]
fn structured_test() {
    let mut rt = Runtime::new().unwrap();

    #[allow(non_snake_case)]
    #[derive(Debug, Deserialize)]
    struct FoundData {
        id: String,
        popularity: Option<u32>,
        inStock: bool,
    }

    let solr_client = DirectSolrClient::new("http://localhost:8983/solr").unwrap();
    let solr_request = solr_client
        .select("techproducts")
        .unwrap()
        .rows(10)
        .q("id:SP251*")
        .fl("id,popularity,inStock");

    let result_struct = rt.block_on(async {
        solr_request
            .call::<SolrSelectType<FoundData>>()
            .await
            .unwrap()
    });

    println!("==== {:#?}", result_struct.response.docs);

    assert_eq!(result_struct.response.docs.len(), 1);
    assert_eq!(result_struct.response.docs[0].popularity, Some(6));
}
