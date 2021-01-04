Stellr
======

A solr client for rust, based heavily on the reqwest and serde crates.

This crate is still very much a work in progress.

Installation
------------

Once this is available on crates.io, you can install it with

```
     [dependencies]
     stellr = "0.1.0"
```

The crate makes use of extension traits, so you should include the stellr::prelude to avoid errors about missing methods

```
     use stellr::prelude::*
```


Usage
-----

This client works in two modes, one with unstructured output (using serde_json::Value), and one where the structure of the output is defined.

Below is an example of an unstructured request (which may return a SolrError, wrapped in a SolrResult)

```
    use stellr::prelude::*;
    use stellr::clients::ZkSolrClient;

    let solr_client = ZkSolrClient::new("localhost:9983", "/")?;
    let solr_request = solr_client
        .select("gettingstarted")?
        .rows(10)
        .q("*:*");

    let result_struct = solr_request.unstructured_call().await?;
    println!("{:?}", result_struct);

    let docs_count = result_struct.get("response")?.get("docs")?.as_array()?.len();
    assert_eq!(10, docs_count);
```

And here is an example of a structured request

```
    use stellr::prelude::*;
    use stellr::clients::DirectSolrClient;
    use stellr::types::SolrSelectType;

    let solr_client = DirectSolrClient::new("localhost", 8983)?;
    let solr_request = solr_client
        .select(collection)?
        .rows(10)
        .q("*:*");

    let solr_response = solr_request.call::<SolrSelectType<String>>().await?;
    if let Ok(result_struct) = response {
        println!("{} results found", result_struct.response.numFound);
    }
```

Please see the examples in cargo doc should you have problems with these.

The client builds heavily on the reqwest and serde crates, in particular using reqwest::RequestBuilder
to construct the query and serde_json to deserialise the output.

The SolrError enum wraps errors generated from these two crates (reqwest::Error and serde_json::error::Error
respectively), along with a catch-all error type for any issues arising in stellr itself.

Default behaviour is to ship async versions of the calls. Specify the "blocking" feature if you want to use
blocking calls (examples above would work with the "await"s removed).

TODO
----

There is still a lot to be done here:

* Test it more, and see how ergonomic it is in practice
* Add more tracing output to the crate
* Support more request types (eg. facet queries and streaming expressions)
* Make the ZkSolrClient watch /live_nodes, and update this in the background
* Generalise this away from Reqwest, so we can support other async runtimes too
