use futures::Future;
pub use openapi_router_derive::*;

pub mod requests;
pub mod responses;

use responses::APIResponse;

// Interface for OpenAPI operations handler
pub trait OpenApiRouter {
    type FutureType: Future<Output = APIResponse> + 'static;

    fn get_operation(
        &self,
        operation_id: String,
    ) -> Box<dyn FnOnce() -> Self::FutureType + Send>;
}
