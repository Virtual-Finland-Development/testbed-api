use app::{
    requests::post_json_request,
    responses::{APIResponse, APIRoutingError},
    router::ParsedRequest,
};
use serde::{Deserialize, Serialize};
use utils::api::get_default_headers;
use utoipa::ToSchema;

use crate::api::routes::testbed::testbed_utils::{
    build_data_product_staged_uri, build_data_product_uri,
};
use serde_json::Value as JSONValue;

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
            external_value = "https://raw.githubusercontent.com/Virtual-Finland/definitions/main/DataProducts/NSG/Agent/BasicInformation_v1.0.json"
        )))
    ))
)]
pub async fn get_nsg_basic_information(request: ParsedRequest) -> APIResponse {
    let request_input: NSGAgentBasicInformationRequest =
        serde_json::from_str(request.body.as_str())?;

    let data_product = "NSG/Agent/BasicInformation_v1.0";
    let data_source = request.query.first("source").unwrap_or("");

    let resolved_data_source = match data_source {
        "" => {
            return Err(APIRoutingError::BadRequest(
                "Missing source parameter".to_string(),
            ));
        }
        "accessfinland" => build_data_product_staged_uri(data_product, data_source),
        _ => build_data_product_uri(data_product, data_source),
    };

    log::debug!("Resolved data source: {}", resolved_data_source);

    let response = post_json_request::<NSGAgentBasicInformationRequest, JSONValue>(
        resolved_data_source,
        &request_input,
        get_default_headers(),
    )
    .await?;

    Ok(response)
}
