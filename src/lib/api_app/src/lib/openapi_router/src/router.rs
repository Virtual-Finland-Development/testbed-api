use http::header::HeaderMap;
use lambda_http::{aws_lambda_events::query_map::QueryMap, Body, Request, RequestExt};

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

pub mod openapi {
    use utoipa::openapi::{OpenApi, PathItem, PathItemType};

    pub fn get_openapi_operation_id(openapi: OpenApi, method: &str, path: &str) -> String {
        let path = openapi.paths.get_path_item(path);
        match path {
            Some(path) => resolve_operation_id(path, method),
            None => "".to_string(),
        }
    }

    fn resolve_operation_id(path: &PathItem, method: &str) -> String {
        let path_item_type = get_path_item_type(method);
        let operationable = path.operations.get(&path_item_type);
        if operationable.is_none() {
            return "".to_string();
        }
        let operation = operationable.unwrap();
        let operation_id = operation.operation_id.clone();
        operation_id.unwrap_or_default()
    }

    fn get_path_item_type(method: &str) -> PathItemType {
        match method {
            "GET" => PathItemType::Get,
            "POST" => PathItemType::Post,
            "PUT" => PathItemType::Put,
            "DELETE" => PathItemType::Delete,
            "OPTIONS" => PathItemType::Options,
            "HEAD" => PathItemType::Head,
            "PATCH" => PathItemType::Patch,
            "TRACE" => PathItemType::Trace,
            _ => PathItemType::Get,
        }
    }
}
