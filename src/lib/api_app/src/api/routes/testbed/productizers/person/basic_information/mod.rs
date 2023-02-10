use crate::api::{
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};

#[utoipa::path(
    get,
    path = "/testbed/productizer/person/basic-information",
    request_body(
        content = Object, 
        description = "Get persons basic information", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/BasicInformation.json",
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Basic information response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/BasicInformation.json",
        )))
    ))
)]
pub async fn get_basic_information(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation";
    let data_source = "virtualfinland";
    let result = super::get_data_product(data_product, data_source, request).await?;
    Ok(result)
}

#[utoipa::path(
    post,
    path = "/testbed/productizer/person/basic-information",
    request_body(
        content = Object, 
        description = "Update persons basic information", 
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/BasicInformation/Write.json",
        )))
    ),
    responses((
        status = 200, 
        body = Object, 
        description = "Basic information response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading.."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/Person/BasicInformation/Write.json",
        ))
    )))
)]
pub async fn write_basic_information(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation/Write";
    let data_source = "virtualfinland";
    let result = super::write_data_product(data_product, data_source, request).await?;
    Ok(result)
}
