use std::cmp::Ordering;
use http::StatusCode;
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

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[allow(non_snake_case)]
struct JobPosting {
    employer: String,
    location: Location,
    basicInfo: BasicInfo,
    publishedAt: String,
    applicationEndDate: String,
    applicationUrl: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Location {
    municipality: String,
    postcode: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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

        log::debug!("Total job postings: {:?}", good_results.len());

        // Uniquefy the results (by mutatation)
        good_results.sort_by(|a, b| job_postings_sort_comparator(a,  b));
        good_results.dedup_by(|a, b| is_job_postings_the_same(a, b));
        let total_count = good_results.len() as i32;

        log::debug!("Merged job postings: {:?}", total_count);

        // Return the response
        let response_output = JobPostingResponse {
            results: good_results,
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


fn job_postings_sort_comparator(a: &JobPosting, b: &JobPosting) -> Ordering {
    if is_job_postings_the_same(a, b) {
        Ordering::Equal
    } else {
        a.publishedAt.cmp(&b.publishedAt)
    }
}

fn is_job_postings_the_same(a: &JobPosting, b: &JobPosting) -> bool {
    a.employer == b.employer &&
    a.location.municipality == b.location.municipality &&
    a.basicInfo.title == b.basicInfo.title &&
    a.publishedAt == b.publishedAt &&
    a.applicationUrl == b.applicationUrl
}