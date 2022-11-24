use http::StatusCode;
use log;
use reqwest;
use serde::{Deserialize, Serialize};
use futures::future;

use crate::api:: {
    routing_types::{APIRoutingError, APIRoutingResponse, ParsedRequest},
    routes::application::resolve_external_service_bad_response,
    utils::get_default_headers,
    text_utils::split_text_keep_right,
};
use super::parse_testbed_request_headers;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)] // FIXME: remove this
struct JobPostingResponse {
    results: Vec<JobPosting>,
    totalCount: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)] 
struct JobPosting {
    employer: String,
    location: Location,
    basicInfo: BasicInfo,
    publishedAt: String,
    applicationEndDate: String,
    applicationUrl: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Location {
    municipality: String,
    postcode: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct BasicInfo {
    title: String,
    description: String,
    workTimeType: String,
}

/**
 * Get job postings
 */
pub async fn find_job_postings(
    request: ParsedRequest,
) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_input = serde_json::from_str(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    let endpoint_urls = vec!["https://gateway.testbed.fi/test/lassipatanen/Job/JobPosting?source=tyomarkkinatori"];

    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    // Get the job postings from the external services using concurrent requests and merge them
    // @see: https://stackoverflow.com/a/51047786
    let client = reqwest::Client::new();
    let response_json_bodies = future::join_all(endpoint_urls.into_iter().map(|endpoint_url| {
        let client = &client;
        let headers = request_headers.clone();
        async move {
            let source = split_text_keep_right(endpoint_url, "?source=");
            let response = client
            .post(endpoint_url)
            .json(&request_input)
            .headers(headers)
            .send()
            .await?;
            log::debug!("Source: {source}, Response: {:#?}", response);
            response.json::<JobPostingResponse>().await
        }
    })).await;

    // Merge the good responses
    // If any response failed, all fail
    let mut response_status = StatusCode::OK;
    let mut error_response_body = String::new();
    let mut response_output = JobPostingResponse {
        results: vec![],
        totalCount: 0,
    };
    
    for r in response_json_bodies {
        match r {
            Ok(r) => {
                // TODO: merge the results
                let mut results = r.results;
                response_output.results.append(&mut results);
                response_output.totalCount += r.totalCount;
            },
            Err(r) => {
                eprintln!("Got an error: {}", r);
                response_status = r.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                error_response_body = r.to_string();
                break;
            },
        }
    }

    if response_status == StatusCode::OK {
        Ok(APIRoutingResponse::new(response_status, &serde_json::to_string(&response_output)?, get_default_headers()))
    } else {
        resolve_external_service_bad_response(response_status, error_response_body)
    }
    
}