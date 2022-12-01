
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

/// Requests many json post requests
/// Returns tuples of good responses and headers along with request url
pub async fn request_post_many_json_requests<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    endpoint_urls: Vec<&str>,
    request_input: &I,
    request_headers: HeaderMap,
    allow_failures: bool,
) -> Result<(StatusCode, Vec::<(O, HeaderMap, String)>, String), APIRoutingError> {
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
   
    let mut good_responses = Vec::<(O, HeaderMap, String)>::new();
    
    for r in response_json_bodies {
        match r {
            Ok(r) => {
                good_responses.push((r.0, r.2, r.3));
            }
            Err(r) => {
                if !allow_failures {
                    // If any response failed, all fail
                    return Ok((r.get_status_code(), Vec::<(O, HeaderMap, String)>::new(), r.to_string()));
                }
            }
        }
    }

    Ok((StatusCode::OK, good_responses, String::new()))
}

/// Request a POST JSON request to an external service
/// Returns the response body as a JSON object, status code, the response headers and request url 
async fn request_post_json_request_data<I: Debug + Serialize, O: Debug + Serialize + for<'a> Deserialize<'a>>(
    client: &reqwest::Client,
    endpoint_url: &str,
    request_input: &I,
    request_headers: HeaderMap,
) -> Result<(O, StatusCode, HeaderMap, String), APIRoutingError> {
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
    let response_headers = response.headers().clone();
    log::debug!("Response status code: {:#?}", response_status);

    if response_status != 200 {
        let response_body = response.text().await.unwrap_or("No response body received".to_string());
        return Err(APIRoutingError::from_status_code_and_message(response_status, response_body.as_str()));
    }

    let response_output = response.json::<O>().await.map_err(|e| {
        log::error!("Error parsing response: {:#?}", e);
        APIRoutingError::UnprocessableEntity("Error parsing response".to_string())
    })?;

    Ok((response_output, response_status, response_headers, endpoint_url.to_string()))
}