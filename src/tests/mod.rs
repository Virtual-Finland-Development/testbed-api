#[cfg(test)]
mod utils_tests {
    use api_app::api::utils::parse_router_request;
    use api_app::lambda_http::{Body, Request};

    #[test]
    fn test_router_request_parsing() {
        let mock_request = Request::new(Body::Text("TEST".to_string()));
        let parsed_request = parse_router_request(mock_request);
        assert_eq!(parsed_request.body, "TEST");
    }
}
