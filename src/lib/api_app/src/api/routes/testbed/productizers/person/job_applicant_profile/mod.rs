use app::{
    router::ParsedRequest,
    responses::APIResponse,
};


#[utoipa::path(
    get,
    path = "/testbed/productizer/person/job-applicant-information",
    request_body(
        content = Object, 
        description = "Get persons job applicant profile", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/JobApplicantProfile.json",
        )))
    ),
    responses((
        status = 200, 
        body = Object, 
        description = "Job applicant profile response", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/JobApplicantProfile.json",
        )))
    )),
    security(( "testbed" = [] ))
)]
pub async fn get_job_applicant_profile(
    request: ParsedRequest,
) -> APIResponse {
    let data_product = "draft/Person/JobApplicantProfile";
    let data_source = "virtualfinland";
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
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/JobApplicantProfile/Write.json",
        )))
    ),
    responses((
        status = 200, 
        body = Object, 
        description = "Job applicant profile response", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/JobApplicantProfile/Write.json",
        )))
    )),
    security(( "testbed" = [] ))
)]
pub async fn write_job_applicant_profile(
    request: ParsedRequest,
) -> APIResponse {
    let data_product = "draft/Person/JobApplicantProfile/Write";
    let data_source = "virtualfinland";
    let result = super::write_data_product(data_product, data_source, request).await?;
    Ok(result)
}
