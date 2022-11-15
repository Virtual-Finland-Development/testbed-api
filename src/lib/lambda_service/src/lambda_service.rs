use api_app::lambda_http::{service_fn, run as lambda_run};
use api_app::api::handler;

pub async fn run() {
    let service = service_fn(handler);
    let _result = lambda_run(service).await;
}
