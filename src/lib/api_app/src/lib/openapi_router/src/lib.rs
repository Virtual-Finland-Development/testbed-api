pub use openapi_router_derive::*;

// Interface for OpenAPI operations handler
pub trait OpenApiRouter {
    fn handle_operation(operation_id: String);
}
