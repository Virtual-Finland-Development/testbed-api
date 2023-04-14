use std::env;

use app::{
    helpers::parse_request_headers, requests::engage_json_request, responses::APIResponse,
    router::ParsedRequest,
};
use http::Method;
use serde_json::{json, Value as JSONValue};

#[utoipa::path(
    delete,
    path = "/users-api/user",
    responses((
        status = 200,
        description = "Deletion response",
    ))
)]
pub async fn delete_user(request: ParsedRequest) -> APIResponse {
    let users_api_origin =
        env::var("USERS_API_ENDPOINT_ORIGIN").expect("USERS_API_ENDPOINT_ORIGIN must be set");
    let endpoint_url = format!("{}/user", users_api_origin);
    let request_input = json!({});
    let request_headers = parse_request_headers(request)?;

    let response = engage_json_request::<JSONValue, JSONValue>(
        Method::DELETE,
        endpoint_url,
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}
