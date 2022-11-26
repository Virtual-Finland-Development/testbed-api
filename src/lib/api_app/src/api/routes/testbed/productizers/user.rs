use http::header::HeaderMap;
use log;
use reqwest;
use serde_json::{Value as JSONValue, json};


use crate::api:: {
    responses::{APIRoutingError, APIRoutingResponse},
    requests::post_json_request,
    utils::ParsedRequest,
};
use super::parse_testbed_request_headers;


pub async fn fetch_user_profile(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = "https://gateway.testbed.fi/test/lassipatanen/User/Profile?source=access_to_finland";
    let request_input = json!({});
    let request_headers = parse_testbed_request_headers(request)?;
    return post_json_request::<JSONValue, JSONValue>(endpoint_url, &request_input, request_headers).await;
}
