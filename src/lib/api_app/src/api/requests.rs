
use http::{header::HeaderMap, StatusCode};
use log;
use reqwest;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

use super:: {
    response_types::{APIRoutingError, APIRoutingResponse},
    utils::get_default_headers
};


pub async fn post_json_request<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    endpoint_url: &str,
    request_input: I,
    request_headers: HeaderMap,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let (response_output, response_status) = get_post_json_request_data::<I, O>(endpoint_url, request_input, request_headers).await?;
    Ok(APIRoutingResponse::new(response_status, &serde_json::to_string(&response_output)?, get_default_headers()))
}

pub async fn get_post_json_request_data<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    endpoint_url: &str,
    request_input: I,
    request_headers: HeaderMap,
) -> Result<(O, StatusCode), APIRoutingError> {
    log::debug!("Url: {:#?}", endpoint_url);
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    let response = reqwest::Client::new()
        .post(endpoint_url)
        .json(&request_input)
        .headers(request_headers)
        .send()
        .await?;

    let response_status = response.status();
    if response_status != 200 {
        let response_body = response.text().await?;
        return Err(APIRoutingError::from_status_code_and_message(response_status, response_body.as_str()));
    }

    let response_output = response.json::<O>().await?;

    Ok((response_output, response_status))
}