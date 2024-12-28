#![allow(clippy::module_name_repetitions)]
use std::result;

use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

pub(crate) type Result<T> = result::Result<T, SiteMapError>;

#[derive(Error, Debug, Diagnostic)]
pub enum SiteMapError {
    #[error("No feed urls were provided. Provide feeds with -s or -S <FILE>.")]
    FeedMissing,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    #[diagnostic(transparent)]
    ChronoError(#[from] ChronoError),
    #[error("The feed at `{0}` was empty.")]
    #[diagnostic(code(sitemap2urllist::empty_feed_error))]
    EmptyFeedError(String),
    #[error("The request feed at `{0}` was rate limited (HTTP 429).")]
    #[diagnostic(code(sitemap2urllist::rate_limit_error))]
    RateLimitError(String),
    #[error("The request feed at `{url}` received an unexpected error (HTTP {status}).")]
    #[diagnostic(code(sitemap2urllist::unexpected_status_error))]
    UnexpectedStatusError { url: String, status: String },
    #[error("Failed to parse URL.")]
    #[diagnostic(code(sitemap2urllist::url_parse_error))]
    UrlParseError(#[from] url::ParseError),
    #[error("Invalid cache file found.")]
    #[diagnostic(code(sitemap2urllist::cache_error))]
    CsvError(#[from] csv::Error),
    #[error("Invalid cache file found.")]
    #[diagnostic(code(sitemap2urllist::cache_error))]
    TryFromIntError(#[from] std::num::TryFromIntError),
}

#[derive(Error, Diagnostic, Debug)]
#[error("Failed to parse datetime.")]
#[diagnostic(code(sitemap2urllist::chrono_error))]
pub struct ChronoError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("this date is invalid")]
    pub span: SourceSpan,
    #[help]
    pub help: String,
}

#[derive(Error, Diagnostic, Debug)]
#[error("Failed to parse feed url.")]
#[diagnostic(code(sitemap2urllist::url_parse_error))]
pub struct FeedUrlError {
    #[source_code]
    pub src: NamedSource<String>,
    #[label("this url is invalid")]
    pub span: SourceSpan,
    #[help]
    pub help: String,
}
