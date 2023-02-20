use api_app::api::handler;
use lambda_http::{run as lambda_run, service_fn};

pub async fn run() {
    let service = service_fn(handler);
    let _result = lambda_run(service).await;
}
