use httpmock::Method::GET;
use httpmock::MockServer;

use stellr::clients::DirectSolrClient;
use stellr::prelude::*;

#[test]
fn blocking_mock_test() {
    let server = MockServer::start();

    let solr_mock = server.mock(|when, then| {
        when.method(GET)
            .path("/solr/films/select")
            .query_param("q", "*:*");
        then.status(200)
            .header("Content-Type", "text/html; charset=UTF-8")
            .body(r#"{ "responseHeader":{ "zkConnected":true, "status":0, "QTime":0, "params":{ "q":"*:*"}}, "response":{"numFound":1100,"start":0,"numFoundExact":true,"docs":[ { "id":"/en/45_2006", "directed_by":["Gary Lennon"], "initial_release_date":"2006-11-30T00:00:00Z", "genre":["Black comedy", "Thriller", "Psychological thriller", "Indie film", "Action Film", "Crime Thriller", "Crime Fiction", "Drama"], "name":".45", "_version_":1683174404568121344}, { "id":"/en/9_2005", "directed_by":["Shane Acker"], "initial_release_date":"2005-04-21T00:00:00Z", "genre":["Computer Animation", "Animation", "Apocalyptic and post-apocalyptic fiction", "Science Fiction", "Short Film", "Thriller", "Fantasy"], "name":"9", "_version_":1683174404581752832}, { "id":"/en/69_2004", "directed_by":["Lee Sang-il"], "initial_release_date":"2004-07-10T00:00:00Z", "genre":["Japanese Movies", "Drama"], "name":"69", "_version_":1683174404582801408}, { "id":"/en/300_2007", "directed_by":["Zack Snyder"], "initial_release_date":"2006-12-09T00:00:00Z", "genre":["Epic film", "Adventure Film", "Fantasy", "Action Film", "Historical fiction", "War film", "Superhero movie", "Historical Epic"], "name":"300", "_version_":1683174404583849984}, { "id":"/en/2046_2004", "directed_by":["Wong Kar-wai"], "initial_release_date":"2004-05-20T00:00:00Z", "genre":["Romance Film", "Fantasy", "Science Fiction", "Drama"], "name":"2046", "_version_":1683174404584898560}, { "id":"/en/quien_es_el_senor_lopez", "directed_by":["Luis Mandoki"], "genre":["Documentary film"], "name":"¿Quién es el señor López?", "_version_":1683174404585947136}, { "id":"/en/weird_al_yankovic_the_ultimate_video_collection", "directed_by":["Jay Levey", "\"Weird Al\" Yankovic"], "initial_release_date":"2003-11-04T00:00:00Z", "genre":["Music video", "Parody"], "name":"\"Weird Al\" Yankovic: The Ultimate Video Collection", "_version_":1683174404586995712}, { "id":"/en/15_park_avenue", "directed_by":["Aparna Sen"], "initial_release_date":"2005-10-27T00:00:00Z", "genre":["Art film", "Romance Film", "Musical", "Drama", "Musical Drama"], "name":"15 Park Avenue", "_version_":1683174404588044288}, { "id":"/en/2_fast_2_furious", "directed_by":["John Singleton"], "initial_release_date":"2003-06-03T00:00:00Z", "genre":["Thriller", "Action Film", "Crime Fiction"], "name":"2 Fast 2 Furious", "_version_":1683174404589092864}, { "id":"/en/7g_rainbow_colony", "directed_by":["Selvaraghavan"], "initial_release_date":"2004-10-15T00:00:00Z", "genre":["Drama"], "name":"7G Rainbow Colony", "_version_":1683174404589092865}] }}"#);
        });

    let solr_client = DirectSolrClient::new(&server.url("/solr")).unwrap();
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
    solr_mock.assert();
    assert_eq!(10, docs_count);
}
