use std::time::Duration;

use clap::{Parser, builder::ValueHint};
use clap_verbosity_flag::Verbosity;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The URL to a sitemap.
    #[arg(value_hint=ValueHint::Url)]
    pub url: Url,
    /// Use request cache stored on disk at `.sitemapcache` (recommended).
    ///
    /// Note that this only prevents refetching if the sitemap source responds
    /// with a 429. In this case, we respect Retry-After, or default to 4h.
    /// Otherwise, the existence of a cache file just allows sitemap2urllist to
    /// respect `ETag` and `Last-Modified` headers for conditional requests.
    #[arg(short, long)]
    pub cache: bool,
    /// Discard all cached requests older than this duration
    #[arg(
        long,
        value_parser = humantime::parse_duration,
        default_value = "14d"
    )]
    pub max_cache_age: Duration,
    #[clap(flatten)]
    pub verbose: Verbosity,
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
