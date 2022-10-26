use lambda_http::{service_fn, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod api;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let service = service_fn(api::handler);

    lambda_http::run(service).await?;
    Ok(())
}
