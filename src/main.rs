use lambda_http::{service_fn, Error};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let service = service_fn(routes::handler);

    lambda_http::run(service).await?;
    Ok(())
}
