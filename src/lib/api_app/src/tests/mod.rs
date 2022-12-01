#[cfg(test)]
mod api_utils_test {
    use crate::api::{
        utils::cut_string_by_delimiter_keep_right,
        routes::testbed::productizers::job::{
            job_models::{JobPostingResponse, JobPosting}, 
            merge_job_posting_results,
            transform_job_posting_results
        }
    };

    #[test]
    fn test_jobs_response_handlings() {
        let mut mock_response = {
            let input_path = "./src/tests/mock_data/job_postings_response.json";
            let text = std::fs::read_to_string(&input_path).unwrap();
            serde_json::from_str::<JobPostingResponse<JobPosting>>(&text).unwrap()
        };
        assert_eq!(mock_response.results.len(), 4);

        // Test mergeing
        let mut transformed_results = transform_job_posting_results("tyomarkkinatori".to_string(), &mut mock_response.results);
        merge_job_posting_results(&mut transformed_results);
        assert_eq!(transformed_results.len(), 3);
    }

    #[test]
    fn string_util_tests() {
        let test_string = "test string";
        assert_eq!(cut_string_by_delimiter_keep_right(test_string.to_string(), " "), "string");
    }
    
}
