use lambda_http::service_fn;

use crate::api;

pub async fn main() {
    let service = service_fn(api::handler);
    let _result = lambda_http::run(service).await;
}
