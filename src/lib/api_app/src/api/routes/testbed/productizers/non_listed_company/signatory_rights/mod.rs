use app::{
    responses::APIResponse,
    router::ParsedRequest
};

#[utoipa::path(
    post,
    path = "/testbed/productizer/non-listed-company/signatory-rights",
    request_body(
        content = Object,
        description = "Get beneficial owners information",
        examples(( "Success" = (
        summary = "JSON example",
        value = json!("Loading..."),
        external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights.json"
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Beneficial owners response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights.json"
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn get_signatory_rights(request: ParsedRequest) -> APIResponse {
    let data_product = "draft/NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights";
    let data_source = "virtualfinland";
    let result = super::post_data_product(data_product, data_source, request).await?;
    Ok(result)
}
