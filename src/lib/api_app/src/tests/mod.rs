#[cfg(test)]
mod api_utils_test {
    use crate::api::routes::testbed::productizers::job::{
        job_models::{JobPostingResponse, JobPosting}, 
        merge_job_posting_results
    };

    #[test]
    fn test_jobs_response_handlings() {
        let mut mock_response = {
            let input_path = "./src/tests/mock_data/job_postings_response.json";
            let text = std::fs::read_to_string(&input_path).unwrap();
            serde_json::from_str::<JobPostingResponse<JobPosting>>(&text).unwrap()
        };
        assert_eq!(mock_response.results.len(), 4);

        // Test merge
        merge_job_posting_results(&mut mock_response.results);
        assert_eq!(mock_response.results.len(), 3);
    }
}
