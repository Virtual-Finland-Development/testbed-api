use self::openapi::get_openapi_operation_id;
use futures::Future;
use http::header::HeaderMap;
use lambda_http::{aws_lambda_events::query_map::QueryMap, Body, Request};
use utoipa::openapi::OpenApi;

use super::responses::APIResponse;
pub use openapi_router_derive::*;
use utils::strings;

// Interface for OpenAPI operations handler
pub trait OpenApiRouter {
    type FutureType: Future<Output = APIResponse> + 'static;

    fn get_operation(
        &self,
        operation_id: String,
        parsed_request: ParsedRequest,
    ) -> Box<dyn FnOnce() -> Self::FutureType + Send>;

    fn run_operation(
        &self,
        operation_id: String,
        parsed_request: ParsedRequest,
    ) -> Self::FutureType {
        let closure = self.get_operation(operation_id, parsed_request);
        closure()
    }

    fn handle(&self, openapi: &OpenApi, parsed_request: ParsedRequest) -> Self::FutureType {
        // Resolve the operation id
        let operation_id = get_openapi_operation_id(
            openapi,
            parsed_request.method.as_str(),
            parsed_request.path.as_str(),
        );
        self.run_operation(operation_id, parsed_request)
    }
}

/**
 * Request input for the router
 */
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
    let uri = request.uri();
    let query_string = uri.query().unwrap_or("");
    let query = query_string.parse::<QueryMap>().unwrap();
    let path = format!("/{}", strings::trim_left_slashes(uri.path()));
    let method = request.method().as_str().to_string();
    let headers = request.headers().clone();

    // Body parsing is left to the route handlers, where the models are defined
    let body: String = match request.body() {
        Body::Text(body) => body.clone(),
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

    pub fn get_openapi_operation_id(openapi: &OpenApi, method: &str, path: &str) -> String {
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
