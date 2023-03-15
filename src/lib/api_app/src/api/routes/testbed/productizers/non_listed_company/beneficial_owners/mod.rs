use app::{
    responses::APIResponse,
    router::ParsedRequest
};

#[utoipa::path(
    post,
    path = "/testbed/productizer/non-listed-company/beneficial-owners",
    request_body(
        content = Object,
        description = "Get beneficial owners information",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/LegalEntity/NonListedCompany/BeneficialOwners.json"
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Beneficial owners response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/LegalEntity/NonListedCompany/BeneficialOwners.json"
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn get_beneficial_owners(request: ParsedRequest) -> APIResponse {
    let data_product = "draft/NSG/Agent/LegalEntity/NonListedCompany/BeneficialOwners";
    let data_source = "virtualfinland";
    let result = super::post_data_product(data_product, data_source, request).await?;
    Ok(result)
}
