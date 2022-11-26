use http::StatusCode;
use itertools::Itertools;
use serde::{ Deserialize, Serialize };
use serde_json::Value as JSONValue;

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response },
    requests::request_post_many_json_requests,
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

    // Fetch the data
    let (response_status, good_responses, error_response_body) = request_post_many_json_requests::<JSONValue, JobPostingResponse>(
        endpoint_urls,
        &request_input,
        request_headers
    ).await.expect("Something went wrong with the bulk requests");

    if response_status == StatusCode::OK {
        
        // Merge the good response results
        let mut good_results = Vec::<JobPosting>::new();
        for mut r in good_responses {
            good_results.append(&mut r.results);
        }

        // Uniquefy the results
        let unique_results = good_results.into_iter().unique_by(|jp| jp.applicationUrl.clone()).collect::<Vec<JobPosting>>();
        let total_count = unique_results.len() as i32;

        // Return the response
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