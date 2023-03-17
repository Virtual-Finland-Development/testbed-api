use serde_json::{json, Value as JSONValue};

use app::{requests::post_json_request, responses::APIResponse, router::ParsedRequest};

use super::{build_data_product_stage_uri, parse_testbed_request_headers};

pub mod basic_information;
pub mod job_applicant_profile;

pub async fn get_data_product(
    data_product: &str,
    data_source: &str,
    request: ParsedRequest,
) -> APIResponse {
    let request_input = json!({});
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        build_data_product_stage_uri(data_product, data_source),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}

pub async fn write_data_product(
    data_product: &str,
    data_source: &str,
    request: ParsedRequest,
) -> APIResponse {
    let request_input: JSONValue =
        serde_json::from_str(request.body.as_str()).unwrap_or_else(|_| json!({}));
    let request_headers = parse_testbed_request_headers(request)?;
    let response = post_json_request::<JSONValue, JSONValue>(
        build_data_product_stage_uri(data_product, data_source),
        &request_input,
        request_headers,
    )
    .await?;
    Ok(response)
}
