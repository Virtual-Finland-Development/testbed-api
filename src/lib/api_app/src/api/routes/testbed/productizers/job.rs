use http::StatusCode;
use log;
use serde::{ Deserialize, Serialize };
use serde_json::Value as JSONValue;
use futures::future;

use crate::api::{
    response_types::{ APIRoutingError, APIRoutingResponse, ParsedRequest },
    requests::get_post_json_request_data,
    routes::application::resolve_external_service_bad_response,
    utils::get_default_headers,
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

    log::debug!("Input: {:#?}", request_input);
    log::debug!("Headers: {:#?}", request_headers);

    // Get the job postings from the external services using concurrent requests and merge them
    // @see: https://stackoverflow.com/a/51047786
    let response_json_bodies = future::join_all(
        endpoint_urls.into_iter().map(|endpoint_url| {
            let headers = request_headers.clone();
            let input = request_input.clone();
            async move {
                get_post_json_request_data::<JSONValue, JobPostingResponse>(endpoint_url, input, headers).await
            }
        })
    ).await;

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
                let response = r.0;
                let mut results = response.results;
                response_output.results.append(&mut results);
                response_output.totalCount += response.totalCount;
            }
            Err(r) => {
                response_status = r.get_status_code();
                error_response_body = r.to_string();
                break;
            }
        }
    }

    if response_status == StatusCode::OK {
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