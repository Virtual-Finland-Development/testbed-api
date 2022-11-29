use std::cmp::Ordering;
use http::StatusCode;
use serde::{ Deserialize, Serialize };

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response },
    requests::request_post_many_json_requests,
    utils::{get_default_headers, ParsedRequest},
};
use super::parse_testbed_request_headers;

//
// Inputs
// 
#[derive(Deserialize, Serialize, Debug)]
struct JobsRequest {
    query: String,
    location: RequestLocation,
    paging: RequestPaging,
}
#[derive(Deserialize, Serialize, Debug)]
struct RequestLocation {
    countries: Vec<String>,
    regions: Vec<String>,
    municipalities: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
struct RequestPaging {
    limit: usize,
    offset: usize,
}

//
// Outputs
//
#[derive(Deserialize, Serialize, Debug)]
pub struct JobPostingResponse {
    pub results: Vec<JobPosting>,
    #[serde(rename = "totalCount")]
    pub total_count: i32,
}
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct JobPosting {
    employer: String,
    location: Location,
    #[serde(rename = "basicInfo")]
    basic_info: BasicInfo,
    #[serde(rename = "publishedAt")]
    published_at: String,
    #[serde(rename = "applicationEndDate")]
    application_end_date: String,
    #[serde(rename = "applicationUrl")]
    application_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Location {
    municipality: String,
    postcode: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct BasicInfo {
    title: String,
    description: String,
    #[serde(rename = "workTimeType")]
    work_time_type: String,
}

/**
 * Get job postings
 */
pub async fn find_job_postings(
    request: ParsedRequest
) -> Result<APIRoutingResponse, APIRoutingError> {
    let mut request_input = serde_json::from_str::<JobsRequest>(request.body.as_str())?;
    let request_headers = parse_testbed_request_headers(request)?;
    let endpoint_urls = vec![
        "https://gateway.testbed.fi/test/lassipatanen/Job/JobPosting?source=tyomarkkinatori",
        "https://gateway.testbed.fi/test/lassipatanen/Job/JobPosting?source=jobs_in_finland"
    ];

    // Compensate the pagination parameters
    //request_input.paging.limit = request_input.paging.limit / endpoint_urls.len();
    request_input.paging.offset = request_input.paging.offset * request_input.paging.limit;

    // Fetch the data
    let (response_status, good_responses, error_response_body) = request_post_many_json_requests::<JobsRequest, JobPostingResponse>(
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

        // Uniquefy the results (with mutatation)
        merge_job_posting_results(&mut good_results);
        let total_count = good_results.len() as i32;

        log::debug!("Merged job postings: {:?}", total_count);

        // Return the response
        let response_output = JobPostingResponse {
            results: good_results,
            total_count: total_count,
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

/**
 * Merge the job posting results
 */
pub fn merge_job_posting_results(results: &mut Vec::<JobPosting>) -> &mut Vec::<JobPosting> {
    results.sort_by(|a, b| job_postings_sort_comparator(a,  b));
    results.dedup_by(|a, b| is_job_postings_the_same(a, b));
    return results
}

fn job_postings_sort_comparator(a: &JobPosting, b: &JobPosting) -> Ordering {
    if is_job_postings_the_same(a, b) {
        Ordering::Equal
    } else {
        a.published_at.cmp(&b.published_at)
    }
}

fn is_job_postings_the_same(a: &JobPosting, b: &JobPosting) -> bool {
    a.employer == b.employer &&
    a.location.municipality == b.location.municipality &&
    a.basic_info.title == b.basic_info.title &&
    a.published_at == b.published_at &&
    a.application_url == b.application_url
}