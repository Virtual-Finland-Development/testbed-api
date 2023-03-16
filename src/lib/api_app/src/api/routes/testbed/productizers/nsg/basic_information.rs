use app::{responses::APIResponse, router::ParsedRequest};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, ToSchema)]
pub struct NSGAgentBasicInformationRequest {
    #[serde(rename = "dataSource")]
    pub data_source: String,
    #[serde(rename = "nationalIdentifier")]
    pub national_identifier: String,
}

#[utoipa::path(
    post,
    path = "/testbed/productizer/nsg/basic-information",
    request_body(
        content = NSGAgentBasicInformationRequest,
        description = "Get company basic information",
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
pub async fn get_basic_information(request: ParsedRequest) -> APIResponse {
    let data_product = "draft/NSG/Agent/BasicInformation";
    let request_input: NSGAgentBasicInformationRequest =
        serde_json::from_str(request.body.as_str())?;

    let mutated_request = ParsedRequest {
        body: serde_json::to_string(&json!({
            "nationalIdentifier": request_input.national_identifier
        }))?,
        ..request
    };

    let result = super::post_data_product(
        data_product,
        request_input.data_source.as_str(),
        mutated_request,
    )
    .await?;
    Ok(result)
}
