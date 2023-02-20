use futures::{future::BoxFuture, FutureExt};
use openapi_router::OpenApiRouter;
use openapi_router::{
    responses::APIResponse,
    router::{openapi::get_openapi_operation_id, ParsedRequest},
};
use utoipa::OpenApi;

pub mod application;
pub mod jmf;
pub mod testbed;

#[derive(OpenApi, OpenApiRouter)]
#[openapi(
    info(
        title = "Testbed API",
        description = "API documentation for the testbed API",
        license(name = "MIT", url = "https://mit-license.org/"),
        version = "0.1.0"
    ),
    paths(
        application::index,
        application::docs,
        application::health_check,
        application::wake_up_external_services,
        testbed::engage_reverse_proxy_request,
        testbed::productizers::figure::get_population,
        testbed::productizers::job::find_job_postings,
        testbed::productizers::user::fetch_user_profile,
        testbed::productizers::user::fetch_user_status_info,
        testbed::productizers::user::update_user_status_info,
        jmf::fetch_jmf_recommendations,
        testbed::productizers::person::basic_information::get_basic_information,
        testbed::productizers::person::basic_information::write_basic_information,
        testbed::productizers::person::job_applicant_profile::get_job_applicant_profile,
        testbed::productizers::person::job_applicant_profile::write_job_applicant_profile
    ),
    components(schemas( // @TODO: would be very nice to auto-generate schemas
        testbed::ProxyRequestInput,
        testbed::productizers::figure::figure_models::PopulationQuery,
        testbed::productizers::figure::figure_models::PopulationResponse,
        testbed::productizers::job::job_models::JobsRequestFromFrontend,
        testbed::productizers::job::job_models::JobPostingResponseForFrontend,
        testbed::productizers::job::job_models::JobPostingForFrontend,
        testbed::productizers::job::job_models::RequestLocation,
        testbed::productizers::job::job_models::RequestRequirements,
        testbed::productizers::job::job_models::RequestPagingFromFrontend,
        testbed::productizers::job::job_models::BasicInfo,
        testbed::productizers::job::job_models::Location,
        jmf::models::RecommendationsRequest,
        jmf::models::RecommendationsResponse,
        jmf::models::Occupation,
        jmf::models::Skill,
    ))
)]
struct Api;

/**
 * API router
 */
pub async fn get_router_response(parsed_request: ParsedRequest) -> APIResponse {
    let openapi = Api::openapi(); // @TODO: ensure as singelton
    match (parsed_request.method.as_str(), parsed_request.path.as_str()) {
        // System routes
        ("OPTIONS", _) => application::cors_preflight_response(parsed_request).await,
        ("GET", "/openapi.json") => {
            application::openapi_spec(openapi.to_json().expect("Failed to parse openapi spec"))
                .await
        }
        // OpenAPI specified routes
        _ => {
            // Resolve the operation id
            let operation_id = get_openapi_operation_id(
                openapi,
                parsed_request.method.as_str(),
                parsed_request.path.as_str(),
            );

            // Exec the operation
            let router = Api;
            router.run_operation(operation_id, parsed_request).await
        }
    }
}
