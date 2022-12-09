use std::env;

use serde_json::{ Value as JSONValue, json };

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse },
    requests::post_json_request,
    utils::ParsedRequest,
};
use super::parse_testbed_request_headers;

pub async fn fetch_user_profile(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    // @TODO: this is a hotfix for the testbed staged source param resolution
    let path_env = env::var("STAGE").expect("STAGE must be set");
    let endpoint_url = match path_env.as_str() {
        "local" =>
            "https://gateway.testbed.fi/test/lassipatanen/User/Profile?source=access_to_finland",
        "dev" =>
            "https://gateway.testbed.fi/test/lassipatanen/User/Profile?source=access_to_finland",
        "staging" =>
            "https://gateway.testbed.fi/test/lassipatanen/User/Profile?source=access_to_finland__staging",
        _ => panic!("Unknown STAGE value"),
    };

    let request_input = json!({}); // Empty body
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        endpoint_url.to_string(),
        &request_input,
        request_headers
    ).await?;
    Ok(response)
}