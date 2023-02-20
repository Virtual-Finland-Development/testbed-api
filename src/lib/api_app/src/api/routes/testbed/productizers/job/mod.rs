use http::{Method, StatusCode};
use math::round;
use std::{cmp::Ordering, collections::hash_map::DefaultHasher, env, hash::Hasher};

use super::parse_testbed_request_headers;

use app::{
    requests::engage_many_json_requests,
    responses::{
        resolve_external_service_bad_response, APIResponse, APIRoutingError, APIRoutingResponse,
    },
    router::ParsedRequest,
};
use utils::{
    api::get_default_headers,
    strings::{cut_string_by_delimiter_keep_right, parse_comma_separated_list},
};

pub mod job_models;
use job_models::{
    JobPosting, JobPostingForFrontend, JobPostingResponse, JobsRequest,
    JobsRequestFromFrontend, ProductizerRequest, RequestPaging,
};

mod job_input_extenders;
use job_input_extenders::extend_job_occupations;

/**
 * Get job postings
 */
#[utoipa::path(
    post,
    path = "/testbed/productizers/find-job-postings",
    request_body(content = JobsRequestFromFrontend, description = "Job postings query"),
    responses((
        status = 200,
        body = JobPostingResponseForFrontend,
        description = "Job postigs response",
    ))
)]
pub async fn find_job_postings(request: ParsedRequest) -> APIResponse {
    let endpoint_urls_as_text = env::var("JOB_POSTING_PRODUCTIZER_ENDPOINTS")
        .expect("JOB_POSTING_PRODUCTIZER_ENDPOINTS must be set");
    let endpoint_urls = parse_comma_separated_list(endpoint_urls_as_text);

    let request = construct_productizer_requests(request, endpoint_urls)?;

    // Fetch the data
    let (response_status, good_responses, error_response_body) =
        engage_many_json_requests::<JobsRequest, JobPostingResponse<JobPosting>>(
            request.endpoint_urls,
            Method::POST,
            &request.request_input,
            request.headers,
            true,
        )
        .await
        .expect("Something went wrong with the bulk requests");

    if response_status == StatusCode::OK {
        // Transform the good response results for the frontend
        let mut good_results = Vec::<JobPostingForFrontend>::new();
        for mut r in good_responses {
            let jobs_source = cut_string_by_delimiter_keep_right(r.2, "?source=");
            let mut transformed_results =
                transform_job_posting_results(jobs_source, &mut r.0.results);
            good_results.append(&mut transformed_results);
        }

        log::debug!("Total job postings: {:?}", good_results.len());

        // Uniquefy the results, transform to a frontend suitable format and sort
        merge_job_posting_results(&mut good_results);
        let merged_total = good_results.len() as i32;
        log::debug!("Merged job postings: {:?}", merged_total);

        let trimmed_results: Vec<&JobPostingForFrontend> = good_results
            .iter()
            .take(request.original_input.paging.items_per_page as usize)
            .collect();
        let final_count = trimmed_results.len() as i32;

        log::debug!("Final total: {:?}", final_count);

        // Return the response
        let response_output = JobPostingResponse {
            results: trimmed_results,
            total_count: final_count,
        };

        Ok(APIRoutingResponse::new(
            response_status,
            &serde_json::to_string(&response_output)?,
            get_default_headers(),
        ))
    } else {
        resolve_external_service_bad_response(response_status, error_response_body)
    }
}

pub fn construct_productizer_requests(
    request: ParsedRequest,
    endpoint_urls: Vec<String>,
) -> Result<ProductizerRequest, APIRoutingError> {
    let original_input =
        serde_json::from_str::<JobsRequestFromFrontend>(request.body.as_str())?;
    let request_input = parse_job_request_input(&original_input);

    let request_headers = parse_testbed_request_headers(request)?;

    // Calc compensated pagination parameters
    let compensated_limit =
        f64::from(request_input.paging.items_per_page) / f64::from(endpoint_urls.len() as i32);
    let rounded_limit = round::ceil(compensated_limit, 0);
    let mut request_limit = rounded_limit as i32;
    if request_limit < 1 {
        request_limit = 1;
    }

    let offset = request_input.paging.page_number * request_limit;

    // Create the request input
    let jobs_request = JobsRequest {
        query: request_input.query,
        location: request_input.location,
        requirements: request_input.requirements,
        paging: RequestPaging {
            limit: request_limit,
            offset,
        },
    };

    Ok(ProductizerRequest {
        endpoint_urls,
        request_input: jobs_request,
        headers: request_headers,
        original_input,
    })
}

pub fn parse_job_request_input(
    request_input: &JobsRequestFromFrontend,
) -> JobsRequestFromFrontend {
    let mut frontend_input = request_input.clone();

    // Parse the requirements
    if frontend_input.requirements.occupations.is_some() {
        let occupations = frontend_input.requirements.occupations.unwrap();
        let extended_occupations = extend_job_occupations(occupations);
        frontend_input.requirements.occupations = Some(extended_occupations);
    }

    frontend_input
}

/**
 * Merge the job posting results, by mutation
 */
pub fn merge_job_posting_results(results: &mut Vec<JobPostingForFrontend>) {
    results.sort_by(job_postings_sort_comparator);
    results.dedup_by(|a, b| is_job_postings_the_same(a, b));
}

fn job_postings_sort_comparator(
    a: &JobPostingForFrontend,
    b: &JobPostingForFrontend,
) -> Ordering {
    if is_job_postings_the_same(a, b) {
        Ordering::Equal
    } else {
        a.published_at.cmp(&b.published_at)
    }
}

fn is_job_postings_the_same(a: &JobPostingForFrontend, b: &JobPostingForFrontend) -> bool {
    a.id == b.id
}

/**
 * Transform the job posting results
 */
pub fn transform_job_posting_results(
    jobs_source: String,
    results: &mut [JobPosting],
) -> Vec<JobPostingForFrontend> {
    results
        .iter_mut()
        .map(|job_posting| JobPostingForFrontend {
            id: generate_job_posting_id(job_posting),
            jobs_source: jobs_source.to_string(),
            employer: job_posting.employer.clone(),
            location: job_posting.location.clone(),
            basic_info: job_posting.basic_info.clone(),
            published_at: job_posting.published_at.clone(),
            application_url: job_posting.application_url.clone(),
            application_end_date: job_posting.application_end_date.clone(),
        })
        .collect()
}

/**
 * Utils
 */

// Generate ID for the job posting

fn generate_job_posting_id(job_posting: &JobPosting) -> String {
    let job_now = job_posting;
    let app_url = job_now.application_url.clone();
    let url_part = app_url.unwrap_or_default();

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
