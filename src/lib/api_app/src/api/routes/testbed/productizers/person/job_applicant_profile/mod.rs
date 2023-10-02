use app::{responses::APIResponse, router::ParsedRequest};

use crate::api::routes::testbed::testbed_utils::get_default_data_product_source;

#[utoipa::path(
    get,
    path = "/testbed/productizer/person/job-applicant-information",
    request_body(
        content = Object, 
        description = "Get persons job applicant profile", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/Person/JobApplicantProfile_v1.0.json",
        )))
    ),
    responses((
        status = 200, 
        body = Object, 
        description = "Job applicant profile response", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/Person/JobApplicantProfile_v1.0.json",
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn get_job_applicant_profile(request: ParsedRequest) -> APIResponse {
    let data_product = "Person/JobApplicantProfile_v1.0";
    let data_source = &get_default_data_product_source();
    let result = super::get_data_product(data_product, data_source, request).await?;
    Ok(result)
}

#[utoipa::path(
    post,
    path = "/testbed/productizer/person/job-applicant-information",
    request_body(
        content = Object, 
        description = "Get persons job applicant profile",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/Person/JobApplicantProfile/Write_v1.0.json",
        )))
    ),
    responses((
        status = 200, 
        body = Object, 
        description = "Job applicant profile response", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/Person/JobApplicantProfile/Write_v1.0.json",
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn write_job_applicant_profile(request: ParsedRequest) -> APIResponse {
    let data_product = "Person/JobApplicantProfile/Write_v1.0";
    let data_source = &get_default_data_product_source();
    let result = super::write_data_product(data_product, data_source, request).await?;
    Ok(result)
}
