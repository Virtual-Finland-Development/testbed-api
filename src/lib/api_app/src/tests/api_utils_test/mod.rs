use mockito::Server;
use std::collections::HashMap;
use std::env;

use crate::api::routes::testbed::productizers::job::{
    construct_productizer_requests,
    job_models::{JobPosting, JobPostingResponse},
    merge_job_posting_results, transform_job_posting_results,
};
use app::router::ParsedRequest;
use http::{
    header::{HeaderMap, HeaderName},
    HeaderValue,
};
use lambda_http::aws_lambda_events::query_map::QueryMap;
use serde_json::json;
use utils::strings::{
    cut_string_by_delimiter_keep_right, parse_comma_separated_list, trim_left_slashes,
};

#[test]
fn test_jobs_response_handlings() {
    let mut mock_response = {
        let input_path = "./src/tests/api_utils_test/mock_data/job_postings_response.json";
        let text = std::fs::read_to_string(input_path).unwrap();
        serde_json::from_str::<JobPostingResponse<JobPosting>>(&text).unwrap()
    };
    assert_eq!(mock_response.results.len(), 4);

    // Test mergeing
    let mut transformed_results = transform_job_posting_results(
        "tyomarkkinatori".to_string(),
        &mut mock_response.results,
    );
    merge_job_posting_results(&mut transformed_results);
    assert_eq!(transformed_results.len(), 3);
}

#[tokio::test]
async fn test_request_parsing() {
    let endpoint_urls = vec![String::from("http1"), String::from("http2")];

    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("authorization"),
        HeaderValue::from_static("Bearer 123"),
    );

    let request_input = ParsedRequest {
        path: "/".to_string(),
        path_params: HashMap::new(),
        method: "POST".to_string(),
        query: QueryMap::default(),
        headers,
        body: json!(
            {
                "query": "",
                "location": {
                    "countries": [],
                    "regions": [],
                    "municipalities": []
                },
                "requirements": {
                    "occupations": [],
                    "skills": [],
                },
                "paging": {
                    "itemsPerPage": 25,
                    "pageNumber": 0
                }
            }
        )
        .to_string(),
    };

    let mut server = Server::new_async().await;
    server.mock("GET", "/resources/OccupationsEscoURL")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!([
                {
                    "uri": "http://data.europa.eu/esco/occupation/0c5e3b5a-1b1f-4b0e-8d0c-4c4c4c4c4c4c",
                }
            ]).to_string())
            .create_async().await;
    env::set_var("CODESETS_BASE_URL", server.url());

    let request = construct_productizer_requests(request_input, endpoint_urls)
        .await
        .expect("Failed to construct the productizer requests");

    assert_eq!(request.endpoint_urls.len(), 2);
    assert_eq!(request.original_input.paging.items_per_page, 25);
    assert_eq!(request.request_input.paging.limit, 13);
}

#[test]
fn string_util_tests() {
    let test_string = "test string";
    assert_eq!(
        cut_string_by_delimiter_keep_right(test_string, " "),
        "string"
    );

    assert_eq!(trim_left_slashes("//test/test"), "test/test");

    assert_eq!(
        parse_comma_separated_list("test1,test2, test3"),
        vec!["test1", "test2", "test3"]
    );
}
