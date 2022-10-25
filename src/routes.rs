use lambda_http::{IntoResponse, Request, RequestExt};

mod population;

pub async fn handler(request: Request) -> Result<impl IntoResponse, std::convert::Infallible> {
    let _context = request.lambda_context();
    population::get_population(request).await
}
