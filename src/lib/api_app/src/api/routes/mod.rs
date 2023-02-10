use super::{
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};
use utoipa::{openapi::PathItemType, OpenApi};

pub fn get_path_item_type(parsed_type: &str) -> PathItemType {
    match parsed_type {
        "get" => PathItemType::Get,
        "post" => PathItemType::Post,
        "put" => PathItemType::Put,
        "delete" => PathItemType::Delete,
        "options" => PathItemType::Options,
        "head" => PathItemType::Head,
        "patch" => PathItemType::Patch,
        "trace" => PathItemType::Trace,
        _ => PathItemType::Get,
    }
}

pub mod application;
pub mod jmf;
pub mod testbed;

/**
 * Exec API routing
 */
pub async fn exec_router_request(parsed_request: ParsedRequest) -> APIRoutingResponse {
    match get_router_response(parsed_request).await {
        Ok(response) => response,
        Err(e) => APIRoutingResponse::from_routing_error(e),
    }
}

#[derive(OpenApi)]
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
    components(schemas( // would be very nice to auto-generate schemas
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
struct ApiDoc;

/**
 * API router
 */
pub async fn get_router_response(
    parsed_request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let openapi = ApiDoc::openapi();

    /*  let path = openapi.paths.get_path_item(parsed_request.path.as_str());
    match path {
        Some(path) => {
            let path_item_type = get_path_item_type(parsed_request.method.as_str());
            let operation = path.operations.get(&path_item_type).expect("BOOM");
            log::debug!("operation: {:?}", operation.operation_id);
        }
        None => {
            log::debug!("Not found: {}", parsed_request.path);
        }
    } */

    match (parsed_request.method.as_str(), parsed_request.path.as_str()) {
        ("OPTIONS", _) => application::cors_preflight_response(parsed_request).await,
        ("GET", "/") => application::index(parsed_request).await,
        ("GET", "/docs") => application::docs(parsed_request).await,
        ("GET", "/openapi.json") => {
            application::openapi_spec(openapi.to_json().expect("Failed to parse openapi spec"))
                .await
        }
        ("GET", "/health") => application::health_check(parsed_request).await,
        ("GET", "/wake-up") => application::wake_up_external_services(parsed_request).await,
        ("POST", "/testbed/reverse-proxy") => {
            testbed::engage_reverse_proxy_request(parsed_request).await
        }
        ("POST", "/testbed/productizers/get-population") => {
            testbed::productizers::figure::get_population(parsed_request).await
        }
        ("POST", "/testbed/productizers/find-job-postings") => {
            testbed::productizers::job::find_job_postings(parsed_request).await
        }
        ("POST", "/testbed/productizers/user-profile") => {
            testbed::productizers::user::fetch_user_profile(parsed_request).await
        }
        ("POST", "/testbed/productizers/fetch-user-status-info") => {
            testbed::productizers::user::fetch_user_status_info(parsed_request).await
        }
        ("POST", "/testbed/productizers/update-user-status-info") => {
            testbed::productizers::user::update_user_status_info(parsed_request).await
        }
        ("POST", "/jmf/recommendations") => {
            jmf::fetch_jmf_recommendations(parsed_request).await
        }

        ("GET", "/testbed/productizer/person/basic-information") => {
            testbed::productizers::person::basic_information::get_basic_information(
                parsed_request,
            )
            .await
        }
        ("POST", "/testbed/productizer/person/basic-information") => {
            testbed::productizers::person::basic_information::write_basic_information(
                parsed_request,
            )
            .await
        }
        ("GET", "/testbed/productizer/person/job-applicant-information") => {
            testbed::productizers::person::job_applicant_profile::get_job_applicant_profile(
                parsed_request,
            )
            .await
        }
        ("POST", "/testbed/productizer/person/job-applicant-information") => {
            testbed::productizers::person::job_applicant_profile::write_job_applicant_profile(
                parsed_request,
            )
            .await
        }

        _ => application::not_found(parsed_request).await,
    }
}
