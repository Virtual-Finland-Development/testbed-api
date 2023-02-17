use openapi_router::{requests::ParsedRequest, responses::APIResponse};
mod openapi_helpers;
use futures::{executor::block_on, future::BoxFuture, FutureExt};
use openapi_helpers::get_openapi_operation_id;
use openapi_router::OpenApiRouter;
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
struct API;

/**
 * API router - // @TODO: would be nice to auto-generate routes from openapi spec
 */
pub async fn get_router_response(parsed_request: ParsedRequest) -> APIResponse {
    let openapi = API::openapi(); // @TODO: ensure as singelton
    let operation_id = get_openapi_operation_id(
        openapi,
        parsed_request.method.as_str(),
        parsed_request.path.as_str(),
    );

    // WIP
    let router = API;
    let closure = router.get_operation(operation_id);
    let future = closure();
    let response = block_on(future);

    response

    /*  match (parsed_request.method.as_str(), parsed_request.path.as_str()) {
        // System routes
        ("OPTIONS", _) => application::cors_preflight_response(parsed_request).await,
        ("GET", "/openapi.json") => {
            application::openapi_spec(
                API::openapi()
                    .to_json()
                    .expect("Failed to parse openapi spec"),
            )
            .await
        }
        // OpenAPI specified routes
        _ => {
            let operation_id = get_openapi_operation_id(
                openapi,
                parsed_request.method.as_str(),
                parsed_request.path.as_str(),
            );

            API::handle_operation(operation_id, parsed_request) // WIP

             match operation_id.as_str() { // @TODO: would be nice to auto-generate this match
                "index" => application::index(parsed_request).await,
                "docs" => application::docs(parsed_request).await,
                "health_check" => application::health_check(parsed_request).await,
                "wake_up_external_services" => application::wake_up_external_services(parsed_request).await,
                "engage_reverse_proxy_request" => testbed::engage_reverse_proxy_request(parsed_request).await,
                "get_population" => testbed::productizers::figure::get_population(parsed_request).await,
                "find_job_postings" => testbed::productizers::job::find_job_postings(parsed_request).await,
                "fetch_user_profile" => testbed::productizers::user::fetch_user_profile(parsed_request).await,
                "fetch_user_status_info" => testbed::productizers::user::fetch_user_status_info(parsed_request).await,
                "update_user_status_info" => testbed::productizers::user::update_user_status_info(parsed_request).await,
                "fetch_jmf_recommendations" => jmf::fetch_jmf_recommendations(parsed_request).await,
                "get_basic_information" => testbed::productizers::person::basic_information::get_basic_information(parsed_request).await,
                "write_basic_information" => testbed::productizers::person::basic_information::write_basic_information(parsed_request).await,
                "get_job_applicant_profile" => testbed::productizers::person::job_applicant_profile::get_job_applicant_profile(parsed_request).await,
                "write_job_applicant_profile" => testbed::productizers::person::job_applicant_profile::write_job_applicant_profile(parsed_request).await,
                _ => application::not_found(parsed_request).await, // Catch all 404
            }
        }
    } */
}
