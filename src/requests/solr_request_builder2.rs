/// New style SolrRequestBuilder - takes a client as a reference, and builds out to a response type
/// response type is pegged at SRBuilder creation
///
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::collections::HashMap;

#[cfg(not(feature = "blocking"))]
use reqwest::RequestBuilder;

#[cfg(feature = "blocking")]
use reqwest::blocking::RequestBuilder;

#[derive(Debug, Clone)]
pub struct SolrRequestBuilder<T> {
    // store a reference to our solr client
    // TODO: should we be sticking this into an Arc?
    solr_client: impl SolrClient,
    // capture the output type with the PhantomData marker (zero-sized)
    output_type: PhantomData<T>,
    // TODO: how do we track the reqwest request builder
    request_builder: Option<RequestBuilder>,
    /*
    path: String,
    params: Option<HashMap<String, String>>,
    // TODO: what should this be?
    method: Option<TODO::HTTPMETHOD>,
    // TODO: should this be structured too?
    payload: Option<String>,
    */
}

impl<T: DeserializeOwned> SolrRequestBuilder<T> {
    pub fn new(solr_client_ref: &impl SolrClient, path_ref: &str) -> SolrRequestBuilder<T> {
        let solr_client = solr_client.clone();
        let path = path_ref.to_string();

        // return a Result, instead of this
        let base_url = solr_client.build_request_url(path).unwrap();

        // TODO: need to parametrise the request methods
        let request_builder = solr_client.get(base_url);)

        SolrRequestBuilder {
            solr_client,
            output_type: PhantomData<T>,
            request_builder,
        }
    }

    // TODO: Handle async/blocking duality
    pub async fn call(self) -> SolrResult<T>
        where T: DeserializeOwned
    {
        let response = self.request_builder.send().await?;
        response.json().await
    }
}

fn generic_select(solr_client_ref: &impl SolrClient) {


}



