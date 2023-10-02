use futures::{future::BoxFuture, FutureExt};
use http::StatusCode;
use serde_json::json;

use crate::api::routes::application;
use app::{
    responses::{APIResponse, APIRoutingResponse},
    router::{parse_router_request, OpenApiRouter, ParsedRequest},
};
use lambda_http::{Body, Request};
use utils::api::{get_default_headers, parse_path_param, parse_query_param};
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/test/{data_product}",
    params(
        ("data_product" = str, Path, description = "Data product name", example = "Weather/Current/Metric"),
        ("source" = str, Query, description = "Data source name", example = "openweather")
    ),
)]
pub async fn get_test_response(request: ParsedRequest) -> APIResponse {
    let path_params = request.path_params.clone();
    let query = request.query;

    let data_product = parse_path_param(path_params, "data_product")?;
    let data_source = parse_query_param(query, "source")?;

    Ok(APIRoutingResponse {
        status_code: StatusCode::OK,
        body: json!({
            "data_source": data_source,
            "data_product": data_product
        })
        .to_string(),
        headers: get_default_headers(),
    })
}

#[derive(OpenApi, OpenApiRouter)]
#[openapi(paths(application::health_check, self::get_test_response))]
struct Api;

pub async fn test_request(path: &str) -> APIRoutingResponse {
    let mock_request = Request::new(Body::Text("".to_string()));
    let (mut parts, body) = mock_request.into_parts();
    parts.uri = path.parse().unwrap();

    let openapi = Api::openapi();
    let router = Api;

    let parsed_request = parse_router_request(Request::from_parts(parts, body));
    let router_request = router.handle(&openapi, parsed_request).await;

    router_request.expect("Should have a response")
}
