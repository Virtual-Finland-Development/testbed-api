use api_app::log::LevelFilter;
use api_app::simple_logger::SimpleLogger;
mod tests;

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
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    loop {
        let _result = service::run().await;

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
