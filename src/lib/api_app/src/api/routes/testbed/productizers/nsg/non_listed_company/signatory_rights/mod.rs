use crate::api::routes::testbed::testbed_utils::service::post_staged_data_product;
use app::{responses::APIResponse, router::ParsedRequest};

#[utoipa::path(
    post,
    path = "/testbed/productizer/non-listed-company/signatory-rights",
    request_body(
        content = Object,
        description = "Get beneficial owners information",
        examples(( "Success" = (
        summary = "JSON example",
        value = json!("Loading..."),
        external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights_v1.0.json"
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Beneficial owners response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights_v1.0.json"
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn get_signatory_rights(request: ParsedRequest) -> APIResponse {
    let data_product = "NSG/Agent/LegalEntity/NonListedCompany/SignatoryRights_v1.0";
    let data_source = "accessfinland";
    let result = post_staged_data_product(data_product, data_source, request).await?;
    Ok(result)
}
