use futures::{future::BoxFuture, FutureExt};
use lazy_static::lazy_static;
use utoipa::OpenApi;

use app::{
    responses::APIResponse,
    router::{OpenApiRouter, ParsedRequest},
};

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
        testbed::get_general_data_product,
        testbed::productizers::figure::get_population,
        testbed::productizers::job::find_job_postings,
        testbed::productizers::user::fetch_user_profile,
        testbed::productizers::user::fetch_user_status_info,
        testbed::productizers::user::update_user_status_info,
        jmf::fetch_jmf_recommendations,
        testbed::productizers::person::basic_information::get_basic_information,
        testbed::productizers::person::basic_information::write_basic_information,
        testbed::productizers::person::job_applicant_profile::get_job_applicant_profile,
        testbed::productizers::person::job_applicant_profile::write_job_applicant_profile,
        testbed::productizers::nsg::non_listed_company::establishment::write_establishment,
        testbed::productizers::nsg::non_listed_company::beneficial_owners::get_beneficial_owners,
        testbed::productizers::nsg::non_listed_company::signatory_rights::get_signatory_rights,
        testbed::productizers::nsg::basic_information::get_nsg_basic_information,
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
        testbed::productizers::nsg::basic_information::NSGAgentBasicInformationRequest,
        jmf::models::RecommendationsRequest,
        jmf::models::RecommendationsResponse,
        jmf::models::Occupation,
        jmf::models::Skill,
    ))
)]
struct Api;

// Create a singleton instance of the router components
lazy_static! {
    static ref OPENAPI_INSTANCE: utoipa::openapi::OpenApi = Api::openapi();
    static ref ROUTER_INSTANCE: Api = Api;
}

/**
 * API router
 */
pub async fn get_router_response(parsed_request: ParsedRequest) -> APIResponse {
    let openapi = &*OPENAPI_INSTANCE;
    let router = &*ROUTER_INSTANCE;

    match (parsed_request.method.as_str(), parsed_request.path.as_str()) {
        // System routes
        ("OPTIONS", _) => application::cors_preflight_response(parsed_request).await,
        ("GET", "/openapi.json") => application::openapi_spec(openapi).await,
        // OpenAPI specified routes
        _ => router.handle(openapi, parsed_request).await,
    }
}
