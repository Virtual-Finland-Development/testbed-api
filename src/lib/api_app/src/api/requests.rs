
use http::{header::HeaderMap, StatusCode};
use log;
use reqwest;
use serde::{Serialize, Deserialize};
use std::fmt::Debug;
use futures::future;

use super:: {
    responses::{APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response},
    utils::get_default_headers
};


pub async fn post_json_request<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    endpoint_url: &str,
    request_input: &I,
    request_headers: HeaderMap,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let client = reqwest::Client::new();
    let response = request_post_json_request_data::<I, O>(&client, endpoint_url, request_input, request_headers).await;
    match response {
        Ok(result) => {
            Ok(APIRoutingResponse::new(result.1, &serde_json::to_string(&result.0)?, get_default_headers()))
        }
        Err(error) => {
            resolve_external_service_bad_response(error.get_status_code(), error.to_string())
        }
    }
}


pub async fn request_post_many_json_requests<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    endpoint_urls: Vec<&str>,
    request_input: &I,
    request_headers: HeaderMap,
) -> Result<(StatusCode, Vec::<O>, String), APIRoutingError> {
    // Get the job postings from the external services using concurrent requests and merge them
    // @see: https://stackoverflow.com/a/51047786
    let api_client = reqwest::Client::new();
    let response_json_bodies = future::join_all(
        endpoint_urls.into_iter().map(|endpoint_url| {
            let client = &api_client;
            let payload = &request_input;
            let headers = request_headers.clone();
            async move {
                request_post_json_request_data::<I, O>(client, endpoint_url, payload, headers).await
            }
        })
    ).await;

    // Merge the good responses
    // If any response failed, all fail
    let mut response_status = StatusCode::OK;
    let mut error_response_body = String::new();
    let mut good_responses = Vec::<O>::new();
    
    for r in response_json_bodies {
        match r {
            Ok(r) => {
                good_responses.push(r.0);
            }
            Err(r) => {
                response_status = r.get_status_code();
                error_response_body = r.to_string();
                break;
            }
        }
    }

    Ok((response_status, good_responses, error_response_body))
}

async fn request_post_json_request_data<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    client: &reqwest::Client,
    endpoint_url: &str,
    request_input: &I,
    request_headers: HeaderMap,
) -> Result<(O, StatusCode), APIRoutingError> {
    log::debug!("Url: {:#?}", endpoint_url);
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    let response = client
        .post(endpoint_url)
        .json(&request_input)
        .headers(request_headers)
        .send()
        .await?;

    let response_status = response.status();
    if response_status != 200 {
        let response_body = response.text().await.unwrap_or("No response body received".to_string());
        return Err(APIRoutingError::from_status_code_and_message(response_status, response_body.as_str()));
    }

    let response_output = response.json::<O>().await?;

    Ok((response_output, response_status))
}