use std::collections::HashMap;

mod mocks;

#[tokio::test]
async fn test_basic_route_resolving() {
    let good_response = mocks::test_request("/health").await;
    assert_eq!(good_response.status_code, 200);
    assert_eq!(good_response.body, "OK");

    let bad_response = mocks::test_request("/wealth").await;
    assert_eq!(bad_response.status_code, 404);
}

#[tokio::test]
async fn test_route_param_resolving() {
    let router_response =
        mocks::test_request("/test/data/product/in/path?source=source-in-query").await;
    assert_eq!(router_response.status_code, 200);

    let parsed_response =
        serde_json::from_str::<HashMap<String, String>>(&router_response.body)
            .expect("Failed to parse response body");
    assert_eq!(parsed_response["data_source"], "source-in-query");
    assert_eq!(parsed_response["data_product"], "data/product/in/path");
}
