use app::{responses::APIResponse, router::ParsedRequest};

#[utoipa::path(
    post,
    path = "/testbed/productizer/non-listed-company/establishment",
    request_body(
        content = Object,
        description = "Write company establishment information",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write.json"
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Company establishment response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write.json"
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn write_establishment(request: ParsedRequest) -> APIResponse {
    let data_product = "draft/NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write";
    let data_source = "virtualfinland";
    let result = super::post_data_product(data_product, data_source, request).await?;
    Ok(result)
}
