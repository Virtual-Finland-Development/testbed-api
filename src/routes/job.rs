use http::Response;
use lambda_http::Request;

pub async fn find_job_postings(
    _request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    Ok(Response::builder()
        .status(200)
        .body("Population".to_string())
        .unwrap())
}
