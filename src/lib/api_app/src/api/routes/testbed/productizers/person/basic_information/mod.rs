use crate::api::{
    responses::{APIRoutingError, APIRoutingResponse},
    utils::ParsedRequest,
};

pub async fn get_basic_information(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation";
    let data_source = "virtualfinland";
    let result = super::get_data_product(data_product, data_source, request).await?;
    Ok(result)
}

pub async fn write_basic_information(request: ParsedRequest) -> Result<APIRoutingResponse, APIRoutingError> {
    let data_product = "draft/Person/BasicInformation/Write";
    let data_source = "virtualfinland";
    let result = super::write_data_product(data_product, data_source, request).await?;
    Ok(result)
}
