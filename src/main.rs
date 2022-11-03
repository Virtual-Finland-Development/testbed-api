use log::LevelFilter;
use simple_logger::SimpleLogger;

mod http_server;
mod lambda_service;

pub mod api;
pub mod tests;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let _result = lambda_service::main();
}
