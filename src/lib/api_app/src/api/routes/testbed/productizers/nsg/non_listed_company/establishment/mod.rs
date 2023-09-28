use crate::api::routes::testbed::testbed_utils::service::post_staged_data_product;
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
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write_v1.0.json"
        )))
    ),
    responses((
        status = 200,
        body = Object,
        description = "Company establishment response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write_v1.0.json"
        )))
    )),
    security(( "BearerAuth" = [] ))
)]
pub async fn write_establishment(request: ParsedRequest) -> APIResponse {
    let data_product = "NSG/Agent/LegalEntity/NonListedCompany/Establishment/Write_v1.0";
    let data_source = "virtualfinland";
    let result = post_staged_data_product(data_product, data_source, request).await?;
    Ok(result)
}
