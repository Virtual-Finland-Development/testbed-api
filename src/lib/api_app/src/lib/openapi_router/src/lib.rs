use futures::Future;
pub use openapi_router_derive::*;

pub mod requests;
pub mod responses;

pub mod router {
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
}

use responses::APIResponse;
use router::ParsedRequest;

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
}
