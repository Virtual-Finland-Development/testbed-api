use lambda_http::Request;

mod routes;

pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    match (request.method(), request.uri().path()) {
        (&lambda_http::http::Method::GET, "/") => {
            return routes::application::index(request).await;
        }
        (&lambda_http::http::Method::GET, "/getPopulation ") => {
            return routes::figure::get_population(request).await;
        }
        (&lambda_http::http::Method::GET, "/findJobPostings ") => {
            return routes::job::find_job_postings(request).await;
        }
        _ => {
            return routes::application::not_found(request).await;
        }
    }
}
