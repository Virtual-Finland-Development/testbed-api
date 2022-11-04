use lambda_http::service_fn;

use api_app::api::handler;

pub async fn run() {
    let service = service_fn(handler);
    let _result = lambda_http::run(service).await;
}
