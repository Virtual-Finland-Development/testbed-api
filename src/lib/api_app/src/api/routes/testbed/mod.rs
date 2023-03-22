use http::HeaderMap;
use serde_json::Value as JSONValue;

use app::{
    responses::{APIResponse, APIRoutingError, APIRoutingResponse},
    router::ParsedRequest,
};

use utils::api::{parse_path_param, parse_query_param};

use crate::api::routes::application::get_external_service_bad_response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utils::api::get_cors_response_headers;
use utoipa::ToSchema;

pub mod productizers;
pub mod testbed_utils;

use testbed_utils::{access_control_check, service::post_data_product};

#[utoipa::path(
    post,
    path = "/testbed/data-product/{data_product}",
    request_body(content = Object, description = "Data product request", example = json!({
        "lat": 60.192059,
        "lon": 24.945831
    })),
    responses((status = 200, body = Object, description = "Data product response")),
    params(
        ("data_product" = str, Path, description = "Data product name", example = "draft/Weather/Current/Metric"),
        ("source" = str, Query, description = "Data source name", example = "openweather")
    ),
)]
pub async fn get_general_data_product(request: ParsedRequest) -> APIResponse {
    log::debug!("Path: {:#?}", request.path);
    log::debug!("Query: {:#?}", request.query);
    log::debug!("Path params: {:#?}", request.path_params);

    let path_params = request.path_params.clone();
    let query = request.query.clone();

    let data_product = parse_path_param(path_params, "data_product")?;
    let data_source = parse_query_param(query, "source")?;

    log::debug!("Data product: {:#?}", data_product);
    log::debug!("Data source: {:#?}", data_source);

    let result =
        post_data_product(data_product.as_str(), data_source.as_str(), request).await?;
    Ok(result)
}

#[derive(Deserialize, Serialize, Debug, ToSchema)]
pub struct ProxyRequestInput {
    method: String,
    url: String,
    body: String,
    headers: HashMap<String, String>,
}

#[utoipa::path(
    post,
    path = "/testbed/reverse-proxy",
    request_body(content = ProxyRequestInput, description = "Proxy request"),
    responses((status = 200, description = "Proxy response", content_type = "application/json"))
)]
pub async fn engage_reverse_proxy_request(request: ParsedRequest) -> APIResponse {
    let request_body_as_text = request.body.as_str();
    log::debug!("Input: {:#?}", request_body_as_text);
    let request_input: ProxyRequestInput =
        serde_json::from_str(request_body_as_text).expect("Failed to parse the request body");

    // Access control list check
    let access_denied = access_control_check(request_input.url.as_str());
    if access_denied {
        return Err(APIRoutingError::Unauthorized(
            "Unknown destination".to_string(),
        ));
    }

    // Transform headers
    let proxy_headers = HeaderMap::try_from(&request_input.headers)?;

    // Execute request
    let response = reqwest::Client::new()
        .post(request_input.url)
        .body(request_input.body)
        .headers(proxy_headers)
        .send()
        .await?;

    log::debug!("Response: {:#?}", response);

    let response_status = response.status();
    if response_status != 200 {
        return get_external_service_bad_response(response).await;
    }

    let response_output = response.json::<JSONValue>().await?;

    Ok(APIRoutingResponse::new(
        response_status,
        &serde_json::to_string(&response_output)?,
        get_cors_response_headers(),
    ))
}
