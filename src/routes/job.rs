use lambda_http::{IntoResponse, Request, RequestExt};

pub async fn find_job_postings(
    request: Request,
) -> Result<impl IntoResponse, std::convert::Infallible> {
    let _context = request.lambda_context();

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters()
            .first("name")
            .unwrap_or_else(|| "stranger")
    ))
}
