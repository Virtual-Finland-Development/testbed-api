use http::header::HeaderMap;
use log;
use reqwest;
use serde_json::Value as JSONValue;

use crate::api:: {
    response_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    routes::application::get_external_service_bad_response,
    utils::get_default_headers
};
use super::parse_testbed_request_headers;


pub async fn fetch_user_profile(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let endpoint_url = "https://gateway.testbed.fi/test/lassipatanen/User/Profile?source=access_to_finland";
    let request_input = serde_json::from_str(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    return post_json_request(endpoint_url, request_input, request_headers).await;
}

async fn post_json_request(
    endpoint_url: &str,
    request_input: JSONValue,
    request_headers: HeaderMap,
) -> Result<APIRoutingResponse, APIRoutingError> {
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    let response = reqwest::Client::new()
        .post(endpoint_url)
        .json(&request_input)
        .headers(request_headers)
        .send()
        .await?;

    log::debug!("Response: {:#?}", response);

    let response_status = response.status();
    if response_status != 200 {
        return get_external_service_bad_response(response).await;
    }

    let response_output = response.json::<JSONValue>().await?;

    Ok(APIRoutingResponse::new(response_status, &serde_json::to_string(&response_output)?, get_default_headers()))
}
