use http::{ header::HeaderMap, StatusCode, Method };
use log;
use reqwest;
use serde::{ Serialize, Deserialize };
use std::fmt::Debug;
use futures::future;
use stopwatch::{ Stopwatch };
use std::time::Duration;

use super::{
    responses::{ APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response },
    utils::get_default_headers,
};

pub async fn post_json_request<
    I: Debug + Serialize,
    O: Debug + Serialize + for<'a> Deserialize<'a>
>(
    endpoint_url: String,
    request_input: &I,
    request_headers: HeaderMap
) -> Result<APIRoutingResponse, APIRoutingError> {
    let client = reqwest::Client::new();
    let response = engage_json_data_request::<I, O>(
        &client,
        endpoint_url,
        Method::POST,
        request_input,
        request_headers
    ).await;
    match response {
        Ok(result) => {
            Ok(
                APIRoutingResponse::new(
                    result.1,
                    &serde_json::to_string(&result.0)?,
                    get_default_headers()
                )
            )
        }
        Err(error) => {
            resolve_external_service_bad_response(error.get_status_code(), error.to_string())
        }
    }
}

/// Requests many json requests
/// Returns tuples of good json responses and headers along with request url
pub async fn engage_many_json_requests<
    I: Debug + Serialize,
    O: Debug + Serialize + for<'a> Deserialize<'a>
>(
    endpoint_urls: Vec<String>,
    request_method: Method,
    request_input: &I,
    request_headers: HeaderMap,
    allow_failures: bool
) -> Result<(StatusCode, Vec<(O, HeaderMap, String)>, String), APIRoutingError> {
    // Get the job postings from the external services using concurrent requests and merge them
    // @see: https://stackoverflow.com/a/51047786
    let api_client = reqwest::Client::new();
    let response_json_bodies = future::join_all(
        endpoint_urls.into_iter().map(|endpoint_url| {
            let client = &api_client;
            let method = request_method.clone();
            let payload = &request_input;
            let headers = request_headers.clone();
            async move {
                engage_json_data_request::<I, O>(
                    client,
                    endpoint_url,
                    method,
                    payload,
                    headers
                ).await
            }
        })
    ).await;

    merge_many_request_responses::<I, O>(response_json_bodies, allow_failures).await
}

// Requests many requests
// Returns tuples of good string responses and headers along with request url
pub async fn engage_many_plain_requests<I: Debug + Serialize>(
    endpoint_urls: Vec<String>,
    request_method: Method,
    request_input: &I,
    request_headers: HeaderMap,
    allow_failures: bool
) -> Result<(StatusCode, Vec<(String, HeaderMap, String)>, String), APIRoutingError> {
    // Get the job postings from the external services using concurrent requests and merge them
    // @see: https://stackoverflow.com/a/51047786
    let api_client = reqwest::Client::new();
    let response_bodies = future::join_all(
        endpoint_urls.into_iter().map(|endpoint_url| {
            let client = &api_client;
            let method = request_method.clone();
            let payload = &request_input;
            let headers = request_headers.clone();
            async move { engage_request::<I>(client, endpoint_url, method, payload, headers).await }
        })
    ).await;

    merge_many_request_responses::<I, String>(response_bodies, allow_failures).await
}

// Merges many request responses
async fn merge_many_request_responses<
    I: Debug + Serialize,
    O: Debug + Serialize + for<'a> Deserialize<'a>
>(
    response_bodies: Vec<Result<(O, StatusCode, HeaderMap, String), APIRoutingError>>,
    allow_failures: bool
) -> Result<(StatusCode, Vec<(O, HeaderMap, String)>, String), APIRoutingError> {
    // Merge the good responses
    let mut good_responses = Vec::<(O, HeaderMap, String)>::new();
    let mut bad_responses = Vec::<(StatusCode, String)>::new();

    for r in response_bodies {
        match r {
            Ok(r) => {
                good_responses.push((r.0, r.2, r.3));
            }
            Err(r) => {
                if !allow_failures {
                    // If any response failed, all fail
                    return Ok((
                        r.get_status_code(),
                        Vec::<(O, HeaderMap, String)>::new(),
                        r.to_string(),
                    ));
                }
                bad_responses.push((r.get_status_code(), r.to_string()));
            }
        }
    }

    if good_responses.is_empty() && !bad_responses.is_empty() {
        // If no responses were good, return the first bad response
        return Ok((
            bad_responses[0].0,
            Vec::<(O, HeaderMap, String)>::new(),
            format!("Error [1/{}]: {}", bad_responses.len(), bad_responses[0].1.to_owned()),
        ));
    }

    Ok((StatusCode::OK, good_responses, String::new()))
}

/// Makes a JSON request to an external service
/// Returns the response body as a JSON object, status code, the response headers and request url
async fn engage_json_data_request<
    I: Debug + Serialize,
    O: Debug + Serialize + for<'a> Deserialize<'a>
>(
    client: &reqwest::Client,
    endpoint_url: String,
    request_method: Method,
    request_input: &I,
    request_headers: HeaderMap
) -> Result<(O, StatusCode, HeaderMap, String), APIRoutingError> {
    let response = engage_request::<I>(
        client,
        endpoint_url,
        request_method,
        request_input,
        request_headers
    ).await?;

    let response_output = serde_json::from_str::<O>(&response.0).map_err(|e| {
        log::error!("Error parsing response: {:#?}", e);
        APIRoutingError::UnprocessableEntity("Error parsing response".to_string())
    })?;

    Ok((response_output, response.1, response.2, response.3))
}

/// Makes a request to an external service
/// Returns the response body as a JSON object, status code, the response headers and request url
async fn engage_request<I: Debug + Serialize>(
    client: &reqwest::Client,
    endpoint_url: String,
    request_method: Method,
    request_input: &I,
    request_headers: HeaderMap
) -> Result<(String, StatusCode, HeaderMap, String), APIRoutingError> {
    log::debug!("Url: {:#?}", endpoint_url);
    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);
    let requested_url = endpoint_url.clone();

    let stopwatch = Stopwatch::start_new();
    let response = client
        .request(request_method, endpoint_url)
        .timeout(Duration::from_secs(29))
        .json(&request_input)
        .headers(request_headers)
        .send().await?;

    let response_status = response.status();
    let response_headers = response.headers().clone();
    let response_body = response.text().await.unwrap_or_else(|_| "No response body received".to_string());

    log::debug!(
        "Request: {}, status code: {:#?}, elapsed time: {}ms",
        requested_url,
        response_status,
        stopwatch.elapsed_ms()
    );

    if response_status != 200 {
        return Err(APIRoutingError::from_status_code_and_message(response_status, response_body));
    }

    Ok((response_body, response_status, response_headers, requested_url))
}