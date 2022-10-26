use lambda_http::Request;

mod application;
mod figure;
mod job;

pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    match (request.method(), request.uri().path()) {
        (&lambda_http::http::Method::GET, "/") => {
            return application::index(request).await;
        }
        (&lambda_http::http::Method::GET, "/getPopulation ") => {
            return figure::get_population(request).await;
        }
        (&lambda_http::http::Method::GET, "/findJobPostings ") => {
            return job::find_job_postings(request).await;
        }
        _ => {
            return application::not_found(request).await;
        }
    }
}
