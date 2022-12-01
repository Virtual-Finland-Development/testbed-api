use std::{cmp::Ordering, collections::hash_map::DefaultHasher, hash::Hasher};
use http::StatusCode;

use crate::api::{
    responses::{ APIRoutingError, APIRoutingResponse, resolve_external_service_bad_response },
    requests::request_post_many_json_requests,
    utils::{get_default_headers, ParsedRequest},
};
use super::parse_testbed_request_headers;

mod job_models;
use job_models::{
    JobsRequest,
    JobPostingResponse,
    JobPosting,
    JobPostingForFrontend,
};

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
    let (response_status, good_responses, error_response_body) = request_post_many_json_requests::<JobsRequest, JobPostingResponse<JobPosting>>(
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

        // Uniquefy the results, transform to a frontend suitable format and sort
        let good_results = merge_job_posting_results(&mut good_results);
        let total_count = good_results.len() as i32;
        log::debug!("Merged job postings: {:?}", total_count);

        // Transform the results to a frontend suitable format
        let transformed_results = transform_job_posting_results(good_results);

        // Return the response
        let response_output = JobPostingResponse {
            results: transformed_results,
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
    generate_job_posting_id(a) == generate_job_posting_id(b)
}

/**
 * Transform the job posting results
 */
pub fn transform_job_posting_results(results: &mut Vec::<JobPosting>) -> Vec::<JobPostingForFrontend> {
    results.into_iter().map(|r| JobPostingForFrontend {
        id: generate_job_posting_id(r),
        employer: r.employer.clone(),
        location: r.location.clone(),
        basic_info: r.basic_info.clone(),
        published_at: r.published_at.clone(),
        application_url: r.application_url.clone(),
        application_end_date: r.application_end_date.clone(),
    }).collect()
}

/**
 * Utils
 */

// Generate ID for the job posting 
fn generate_job_posting_id(job_posting: &JobPosting) -> String {
    let job_now = job_posting.clone();
    let app_url = job_now.application_url.clone();
    let url_part = app_url.unwrap_or("".to_string());

    let mut hasher = DefaultHasher::new();
    let mut id_parts = String::new();
    id_parts.push_str(job_now.employer.as_str());
    id_parts.push_str(job_now.location.municipality.as_str());
    id_parts.push_str(job_now.basic_info.title.as_str());
    id_parts.push_str(job_now.published_at.as_str());
    id_parts.push_str(url_part.as_str());
    hasher.write(id_parts.as_bytes());
    hasher.finish().to_string()
}