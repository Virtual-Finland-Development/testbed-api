use http::header::HeaderMap;
use http::Response;
use lambda_http::Request;
use serde_json::Value;

mod routes;

pub struct APIRoutingResponse {
    pub status_code: u16,
    pub body: Value, // Only json values are supported
    pub headers: HeaderMap,
}

/**
 * The handler function for the lambda.
 */
pub async fn handler(
    request: Request,
) -> Result<lambda_http::Response<String>, std::convert::Infallible> {
    let router_response = exec_router_request(request).await;
    let api_response = Response::builder()
        .status(router_response.status_code)
        .body(router_response.body.to_string())
        .unwrap();

    Ok(api_response)
}

/**
 * This is the router for the API.
 */
async fn exec_router_request(request: Request) -> APIRoutingResponse {
    let path = request.uri().path();
    let method = request.method();

    match (method, path) {
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
