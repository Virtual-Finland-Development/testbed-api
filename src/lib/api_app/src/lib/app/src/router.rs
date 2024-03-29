use std::collections::HashMap;

use self::openapi::resolve_operation;
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
        let operation = resolve_operation(
            openapi,
            parsed_request.method.as_str(),
            parsed_request.path.as_str(),
        );
        self.run_operation(
            operation.operation_id,
            ParsedRequest {
                path_params: operation.path_params,
                ..parsed_request
            },
        )
    }
}

/**
 * Request input for the router
 */
#[derive(Debug)]
pub struct ParsedRequest {
    pub path: String,
    pub path_params: HashMap<String, String>,
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
    let query = uri
        .query()
        .unwrap_or("")
        .parse::<QueryMap>()
        .expect("Failed to parse query string");
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
        path_params: HashMap::new(),
        method,
        query,
        headers,
        body,
    }
}

pub mod openapi {
    use regex::Regex;
    use std::collections::HashMap;
    use urlencoding::decode as url_decode;
    use utoipa::openapi::{OpenApi, PathItem, PathItemType};

    pub struct OperationResolution {
        pub operation_id: String,
        pub path_params: HashMap<String, String>,
    }

    pub fn resolve_operation(
        openapi: &OpenApi,
        method: &str,
        path: &str,
    ) -> OperationResolution {
        log::debug!("Path: {:?}", path);

        for (key, value) in openapi.paths.paths.iter() {
            if key == path {
                return OperationResolution {
                    operation_id: resolve_operation_id(value, method),
                    path_params: HashMap::new(),
                };
            }

            if key.contains('{') && key.contains('}') {
                let re = Regex::new(r"\{(\w+)\}").unwrap();
                let groups: Vec<_> = re
                    .captures_iter(key)
                    .map(|caps| caps[1].to_string())
                    .collect();

                let re_str = re.replace_all(key, "(.+)").to_string();
                let input_re = Regex::new(&re_str).unwrap();
                let input_caps = input_re.captures(path);

                match input_caps {
                    Some(caps) => {
                        let values: Vec<_> =
                            groups.iter().map(|_group| caps[1].to_string()).collect();

                        if groups.len() != values.len() {
                            continue;
                        }

                        let map: HashMap<_, _> = groups
                            .iter()
                            .zip(values.iter())
                            .map(|(key, value)| {
                                (
                                    key.to_string(),
                                    url_decode(value.as_str())
                                        .expect("Failed to url-decode the path param")
                                        .into_owned(),
                                )
                            })
                            .collect();

                        return OperationResolution {
                            operation_id: resolve_operation_id(value, method),
                            path_params: map,
                        };
                    }
                    None => {
                        continue;
                    }
                }
            }
        }

        OperationResolution {
            operation_id: String::from(""),
            path_params: HashMap::new(),
        }
    }

    fn resolve_operation_id(path_item: &PathItem, method: &str) -> String {
        let path_item_type = get_path_item_type(method);
        let operationable = path_item.operations.get(&path_item_type);
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
