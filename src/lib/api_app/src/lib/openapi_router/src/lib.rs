use futures::Future;
pub use openapi_router_derive::*;

pub mod requests;
pub mod responses;

use requests::ParsedRequest;
use responses::APIResponse;

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
