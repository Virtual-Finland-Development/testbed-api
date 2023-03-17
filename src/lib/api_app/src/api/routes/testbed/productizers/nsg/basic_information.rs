use crate::api::routes::testbed::testbed_utils::service::post_data_product;
use app::{
    responses::{APIResponse, APIRoutingError},
    router::ParsedRequest,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, ToSchema)]
pub struct NSGAgentBasicInformationRequest {
    #[serde(rename = "nationalIdentifier")]
    pub national_identifier: String,
}

#[utoipa::path(
    post,
    path = "/testbed/productizer/nsg/basic-information",
    request_body(
        content = NSGAgentBasicInformationRequest,
        description = "Get company basic information input",
        examples(
            (
                "Success" = (
                    summary = "National identifier",
                    value = json!({
                        "nationalIdentifier": "2464491-9"
                    }),
                )
            )
        )
    ),
    params(
        ("source" = String, Query, description = "Data source: fi, se, no..", example = "fi"),
    ),
    responses((
        status = 200,
        body = Object,
        description = "Basic information response",
        examples(( "Success" = (
            summary = "JSON example",
            value = json!("Loading..."),
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/draft/NSG/Agent/BasicInformation.json"
        )))
    ))
)]
pub async fn get_nsg_basic_information(request: ParsedRequest) -> APIResponse {
    let data_product = "draft/NSG/Agent/BasicInformation";
    let query = request.query.clone();
    let data_source = query.first("source").unwrap_or("");
    if data_source.is_empty() {
        return Err(APIRoutingError::BadRequest(
            "Missing source parameter".to_string(),
        ));
    }

    post_data_product(data_product, data_source, request).await
}
