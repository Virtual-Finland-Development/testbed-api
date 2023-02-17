use http::header::HeaderMap;
use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::{Body, Request, RequestExt};
use utils::strings;

pub struct ParsedRequest {
    pub path: String,
    pub method: String,
    pub query: QueryMap,
    pub headers: HeaderMap,
    pub body: String,
}

/**
 * Convert the lambda_http::Request to a parsed_request.
 */
pub fn parse_router_request(request: Request) -> ParsedRequest {
    let path = format!("/{}", strings::trim_left_slashes(request.uri().path()));
    let method = request.method().as_str().to_string();
    let query = request.query_string_parameters();
    let headers = request.headers().clone();

    // Body parsing is left to the route handlers, where the models are defined
    let body: String = match request.body() {
        Body::Text(body) => body.clone(),
        //Body::Binary(body) => serde_json::from_slice(body),
        _ => "".to_string(),
    };

    ParsedRequest {
        path,
        method,
        query,
        headers,
        body,
    }
}
