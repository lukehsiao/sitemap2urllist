use std::result;

use miette::Diagnostic;
use thiserror::Error;

pub(crate) type Result<T> = result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("the sitemap at `{0}` was empty.")]
    #[diagnostic(code(sitemap2urllist::empty_sitemap))]
    EmptySitemap(String),

    #[error("the request to `{0}` was rate limited (HTTP 429).")]
    #[diagnostic(code(sitemap2urllist::rate_limit))]
    RateLimit(String),

    #[error("the sitemap at `{url}` is too large ({bytes} bytes).")]
    #[diagnostic(code(sitemap2urllist::sitemap_too_large))]
    SitemapTooLarge { url: String, bytes: u64 },

    #[error("`{url}` received an unexpected status (HTTP {status}).")]
    #[diagnostic(code(sitemap2urllist::unexpected_status))]
    UnexpectedStatus { url: String, status: String },

    #[error("invalid XML, is `{url}` a valid XML sitemap?")]
    #[diagnostic(code(sitemap2urllist::invalid_xml))]
    InvalidXml {
        url: String,
        #[source]
        source: quick_xml::DeError,
    },

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    #[diagnostic(code(sitemap2urllist::xml_error))]
    Xml(#[from] quick_xml::DeError),

    #[error(transparent)]
    #[diagnostic(code(sitemap2urllist::cache_error))]
    SerdeJson(#[from] serde_json::Error),

    #[error("failed to read or write a file.")]
    #[diagnostic(code(sitemap2urllist::io_error))]
    Io(#[from] std::io::Error),

    #[error("failed to parse URL.")]
    #[diagnostic(code(sitemap2urllist::url_parse_error))]
    UrlParse(#[from] url::ParseError),
}
