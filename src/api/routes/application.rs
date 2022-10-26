use http::Response;
use lambda_http::Request;
use serde_json::json;

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
    let response = json!({
        "message": "Not Found".to_string(),
    });

    Ok(Response::builder()
        .status(200)
        .body(response.to_string())
        .unwrap())
}
