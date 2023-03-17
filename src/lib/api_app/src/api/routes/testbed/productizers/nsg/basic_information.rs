use app::{
    requests::post_json_request,
    responses::{APIResponse, APIRoutingError},
    router::ParsedRequest,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use utils::api::get_default_headers;
use utoipa::ToSchema;

use crate::api::routes::testbed::productizers::build_data_product_uri;

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
                    summary = "Nationa identifier",
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
    let request_input: NSGAgentBasicInformationRequest =
        serde_json::from_str(request.body.as_str())?;

    let data_product = "draft/NSG/Agent/BasicInformation";
    let data_source = request.query.first("source").unwrap_or("");
    if data_source.is_empty() {
        return Err(APIRoutingError::BadRequest(
            "Missing source parameter".to_string(),
        ));
    }

    let response = post_json_request::<NSGAgentBasicInformationRequest, JSONValue>(
        build_data_product_uri(data_product, data_source),
        &request_input,
        get_default_headers(),
    )
    .await?;

    Ok(response)
}
