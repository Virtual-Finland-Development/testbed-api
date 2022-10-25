use lambda_http::IntoResponse;

pub async fn index() -> Result<impl IntoResponse, std::convert::Infallible> {
    Ok("Root path")
}

pub async fn not_found() -> Result<impl IntoResponse, std::convert::Infallible> {
    Ok("Not found")
}
