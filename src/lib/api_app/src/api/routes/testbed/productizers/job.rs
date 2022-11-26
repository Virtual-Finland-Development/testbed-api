use http::StatusCode;
use itertools::Itertools;
use serde::{ Deserialize, Serialize };
use serde_json::Value as JSONValue;
use futures::future;

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response },
    requests::request_post_json_request_data,
    utils::{get_default_headers, ParsedRequest},
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
    applicationUrl: String,
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
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let request_input = serde_json::from_str::<JSONValue>(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    let endpoint_urls = vec![
        "https://gateway.testbed.fi/test/lassipatanen/Job/JobPosting?source=tyomarkkinatori"
    ];

    // Get the job postings from the external services using concurrent requests and merge them
    // @see: https://stackoverflow.com/a/51047786
    let api_client = reqwest::Client::new();
    let response_json_bodies = future::join_all(
        endpoint_urls.into_iter().map(|endpoint_url| {
            let client = &api_client;
            let payload = request_input.clone();
            let headers = request_headers.clone();
            async move {
                request_post_json_request_data::<JSONValue, JobPostingResponse>(client, endpoint_url, payload, headers).await
            }
        })
    ).await;

    // Merge the good responses
    // If any response failed, all fail
    let mut response_status = StatusCode::OK;
    let mut error_response_body = String::new();
    let mut good_results = Vec::<JobPosting>::new();
    
    for r in response_json_bodies {
        match r {
            Ok(r) => {
                // TODO: merge the results
                let response = r.0;
                let mut results = response.results;
                good_results.append(&mut results);
            }
            Err(r) => {
                response_status = r.get_status_code();
                error_response_body = r.to_string();
                break;
            }
        }
    }

    if response_status == StatusCode::OK {
        
        // Merge the good results
        let unique_results = good_results.into_iter().unique_by(|r| r.applicationUrl.clone()).collect::<Vec<JobPosting>>();
        let total_count = unique_results.len() as i32;

        let response_output = JobPostingResponse {
            results: unique_results,
            totalCount: total_count,
        };

        Ok(
            APIRoutingResponse::new(
                response_status,
                &serde_json::to_string(&response_output)?,
                get_default_headers()
            )
        )
    } else {
        resolve_external_service_bad_response(response_status, error_response_body)
    }
}