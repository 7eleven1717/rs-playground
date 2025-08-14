mod cli;

use clap::Parser;
use cli::Cli;

#[cfg(feature = "tracing")]
use tracing::{Level, info};
#[cfg(feature = "tracing")]
use tracing_subscriber::fmt::format::FmtSpan;

use hello_world::hello;

fn main() {
    let cli = Cli::parse();

    #[cfg(feature = "tracing")]
    if !cli.quiet {
        let level = match cli.debug {
            0 => Level::INFO,
            1 => Level::DEBUG,
            _ => Level::TRACE,
        };
        tracing_subscriber::fmt()
            .with_max_level(level)
            .with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
            .init();
        info!("Tracing level [{}]", &level);
    }

    hello();
}
