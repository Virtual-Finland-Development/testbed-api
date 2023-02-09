use crate::api::{
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};

pub async fn get_job_applicant_profile(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/JobApplicantProfile";
    let data_source = "virtualfinland";
    let result = super::get_data_product(data_product, data_source, request).await?;
    Ok(result)
}

pub async fn post_job_applicant_profile(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/JobApplicantProfile/Write";
    let data_source = "virtualfinland";
    let result = super::write_data_product(data_product, data_source, request).await?;
    Ok(result)
}

