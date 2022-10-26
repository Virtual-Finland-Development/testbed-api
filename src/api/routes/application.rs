use http::Response;
use lambda_http::Request;

pub async fn index(
    _request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    Ok(Response::builder()
        .status(200)
        .body("Index".to_string())
        .unwrap())
}

pub async fn not_found(
    _request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    Ok(Response::builder()
        .status(200)
        .body("Not found".to_string())
        .unwrap())
}
