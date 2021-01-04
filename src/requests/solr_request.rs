use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::errors::{SolrError, SolrResult};

#[cfg(not(feature = "blocking"))]
use reqwest::RequestBuilder;

#[cfg(not(feature = "blocking"))]
use async_trait::async_trait;

#[cfg(feature = "blocking")]
use reqwest::blocking::RequestBuilder;

use tracing::error;

/// Common methods for solr requests
///
/// The most important method here is "call" which outputs a deserialised instance. The other
/// method is unstructured_call(), which hard-codes the output to be a serde_json::Value.
///
/// The SolrRequest trait is applied to the reqwest::RequestBuilder, as an extension trait (hence
/// it is important to use the stellr prelude).
///
/// There are two versions of this trait, one for async requests, and one for blocking requests.
/// This is controlled by the "blocking" feature for the stellr crate.
///
/// Helper structs for deserialisation are found under request::types, to simplify the legwork
/// with common tasks.
///
/// ## Errors
///
/// If the response is not valid JSON (eg. if it return a stack trace or an HTML error page) then a
/// SolrError::ResponseParseError will be returned, which will contain the underlying serde error
/// message.
#[cfg(not(feature = "blocking"))]
#[async_trait]
pub trait SolrRequest {
    /// Convert the solr response into a user-defined struct (implementing the Deserialise trait)
    async fn call<T: DeserializeOwned>(self) -> SolrResult<T>;
    /// Convert the solr response into a serde_json::Value struct
    async fn unstructured_call(self) -> SolrResult<Value>;
}

#[cfg(feature = "blocking")]
pub trait SolrRequest {
    /// Make a request from a supplied requestBuilder, and deserialise that to a user specified
    /// struct
    fn call<T: DeserializeOwned>(self) -> SolrResult<T>;

    /// Make a request and deserialise the data into a serde_json::Value instance
    fn unstructured_call(self) -> SolrResult<Value>;
}

#[cfg(not(feature = "blocking"))]
#[async_trait]
impl SolrRequest for RequestBuilder {
    async fn call<T: DeserializeOwned>(self) -> SolrResult<T> {
        let response = self.send().await?;

        let body_text = response.text().await?;

        parse_json(&body_text)
    }

    async fn unstructured_call(self) -> SolrResult<Value> {
        self.call::<Value>().await
    }
}

#[cfg(feature = "blocking")]
impl SolrRequest for RequestBuilder {
    fn call<T>(self) -> SolrResult<T>
    where
        T: DeserializeOwned,
    {
        let response = self.send()?;

        let body_text = response.text()?;

        parse_json(&body_text)
    }

    fn unstructured_call(self) -> SolrResult<Value> {
        self.call::<Value>()
    }
}

fn parse_json<T: DeserializeOwned>(body_text: &str) -> SolrResult<T> {
    match serde_json::from_str::<T>(body_text) {
        Ok(v) => Ok(v),
        Err(e) => {
            error!("Failed to decode the response: {:#?}", &body_text);
            Err(SolrError::ResponseParseError(e))
        }
    }
}
