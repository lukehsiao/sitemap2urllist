use clap::Parser;
use miette::{Context, Result};
use std::io;
use tracing_log::AsTrace;

use sitemap2urllist::{self, args::Args};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(format!(
            "sitemap2urllist={}",
            args.verbose.log_level_filter().as_trace()
        ))
        .with_writer(io::stderr)
        .init();
    // I feel like I shouldn't need wrap_err, but it doesn't work without it.
    sitemap2urllist::run(args).await.wrap_err("runtime error")
}
