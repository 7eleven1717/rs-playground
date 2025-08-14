#[unsafe(no_mangle)]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(feature = "tracing")]
use tracing::{Level, debug, error, span, warn};

pub fn hello() {
    #[cfg(feature = "tracing")]
    let info_span = span!(Level::INFO, "info_span").entered();
    #[cfg(feature = "tracing")]
    let debug = span!(Level::DEBUG, "debug_span").entered();
    #[cfg(feature = "tracing")]
    let trace = span!(Level::TRACE, "trace_span").entered();

    #[cfg(feature = "tracing")]
    error!("This is an error message.");
}
