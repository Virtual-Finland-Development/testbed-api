#[cfg(test)]
mod api_utils_test {
    use serde_json::json;
    use http::{ HeaderValue, header::{ HeaderMap, HeaderName } };
    use lambda_http::aws_lambda_events::query_map::QueryMap;
    use crate::api::{
        utils::{ cut_string_by_delimiter_keep_right, ParsedRequest },
        routes::testbed::productizers::job::{
            job_models::{ JobPostingResponse, JobPosting },
            merge_job_posting_results,
            transform_job_posting_results,
            construct_productizer_requests,
        },
    };

    #[test]
    fn test_jobs_response_handlings() {
        let mut mock_response = {
            let input_path = "./src/tests/mock_data/job_postings_response.json";
            let text = std::fs::read_to_string(&input_path).unwrap();
            serde_json::from_str::<JobPostingResponse<JobPosting>>(&text).unwrap()
        };
        assert_eq!(mock_response.results.len(), 4);

        // Test mergeing
        let mut transformed_results = transform_job_posting_results(
            "tyomarkkinatori".to_string(),
            &mut mock_response.results
        );
        merge_job_posting_results(&mut transformed_results);
        assert_eq!(transformed_results.len(), 3);
    }

    #[test]
    fn test_request_parsing() {
        let endpoint_urls = vec!["http1", "http2"];

        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("authorization"),
            HeaderValue::from_static("Bearer 123")
        );

        let request_input = ParsedRequest {
            path: "/".to_string(),
            method: "POST".to_string(),
            query: QueryMap::default(),
            headers: headers,
            body: json!(
                {
                    "query": "",
                    "location": {
                        "countries": vec![""],
                        "regions": vec![""],
                        "municipalities": vec![""]
                    },
                    "paging": {
                        "items_per_page": 25,
                        "page_number": 0
                    }
                }
            ).to_string(),
        };

        let request = construct_productizer_requests(request_input, endpoint_urls).expect(
            "Failed to construct the productizer requests"
        );

        assert_eq!(request.endpoint_urls.len(), 2);
        assert_eq!(request.original_input.paging.items_per_page, 25);
        assert_eq!(request.request_input.paging.limit, 13);
    }

    #[test]
    fn string_util_tests() {
        let test_string = "test string";
        assert_eq!(cut_string_by_delimiter_keep_right(test_string.to_string(), " "), "string");
    }
}