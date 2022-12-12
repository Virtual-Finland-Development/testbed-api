use std::str::FromStr;

use api_app::log::LevelFilter;
use api_app::simple_logger::SimpleLogger;
mod tests;
#[cfg(feature = "local-dev")]
use dotenv;
#[cfg(feature = "local-dev")]
use http_server as service;
#[cfg(not(feature = "local-dev"))]
use lambda_service as service;

// Hot reloading for local development
//
// @see: https://github.com/rksm/rust-hot-reload
//
#[cfg(feature = "local-dev")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    // get all public #[no_mangle] functions from that file and generate
    // functions with the same signatures that are hot-reloadable.
    hot_functions_from_file!("src/lib/http_server/src/http_server.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "local-dev")]
    {
        let stage = std::env::var("STAGE").unwrap_or_else(|_| "local".to_string());
        dotenv::from_filename(format!(".env.{}", stage)).ok(); // override with stage specific env if any
        dotenv::from_filename(".env").ok();
    }

    // Initialize the logger
    let logging_level: LevelFilter = match std::env::var("LOGGING_LEVEL") {
        Ok(level) => LevelFilter::from_str(level.as_ref()).expect("Invalid logging level"),
        Err(_) => LevelFilter::Info,
    };

    SimpleLogger::new().with_level(logging_level).init().unwrap();

    #[allow(clippy::never_loop)] // Allow the loop to be skipped in a not-local dev
    loop {
        service::run().await;

        #[cfg(not(feature = "local-dev"))]
        break;
        #[cfg(feature = "local-dev")]
        {
            // Wait until a library change happens (but the old lib is still loader)
            let token = hot_lib::subscribe().wait_for_about_to_reload();
            // while token exists, reload is blocked
            drop(token);

            // wait for reload to be done
            hot_lib::subscribe().wait_for_reload();
        }
    }
}