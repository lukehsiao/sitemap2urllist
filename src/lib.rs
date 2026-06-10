pub mod args;
pub mod cache;
pub mod error;
pub mod fetcher;
pub mod sitemap;

use std::{
    collections::HashSet,
    io::{self, BufWriter, Write},
    sync::Arc,
};

use reqwest::Client;
use tokio::{sync::Semaphore, task::JoinSet};
use tracing::{debug, warn};
use url::Url;

use crate::{
    args::Args,
    cache::{Cache, CachePath},
    error::Result,
    fetcher::Fetcher,
    sitemap::{Parsed, UrlSet, parse_sitemap, parse_urlset},
};

/// Cap on in-flight fetches so a huge sitemap index cannot exhaust the
/// process's file descriptors (macOS defaults to 256 per process, and the
/// sitemap spec allows 50,000 children per index). 32 keeps the network
/// saturated while staying well below that floor.
const MAX_CONCURRENT_FETCHES: usize = 32;

/// Collect every URL location from all the `UrlSet`s, deduplicated and sorted so
/// the output is deterministic across runs (the input order from concurrent
/// fetches is not).
fn collect_urls(urlsets: &[UrlSet]) -> Vec<String> {
    let unique: HashSet<String> = urlsets
        .iter()
        .flat_map(|urlset| urlset.urls.iter().map(|url| url.location.to_string()))
        .collect();
    let mut urls: Vec<String> = unique.into_iter().collect();
    urls.sort_unstable();
    urls
}

/// Write the URLs to `w`, one per line.
///
/// A closed pipe (e.g. `sitemap2urllist ... | head`) is treated as success:
/// the reader has everything it wants, and Unix convention is to exit quietly
/// rather than panic the way `println!` does on EPIPE.
fn write_urls(mut w: impl Write, urls: &[String]) -> Result<()> {
    let written = urls
        .iter()
        .try_for_each(|url| writeln!(w, "{url}"))
        .and_then(|()| w.flush());
    match written {
        Err(e) if e.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => Ok(result?),
    }
}

async fn get_urlsets(url: &Url, client: &Client, cache: &Arc<Cache>) -> Result<Vec<UrlSet>> {
    let body = url.fetch(client, cache).await?;

    // Google enforces that sitemap indexes cannot contain other sitemap indices,
    // so we don't go deeper than 1 level: a sitemap index's children are fetched
    // concurrently and parsed strictly as url sets.
    match parse_sitemap(url, &body)? {
        Parsed::Index(index) => {
            let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_FETCHES));
            let mut tasks = JoinSet::new();
            for ptr in index.sitemaps {
                let cache = cache.clone();
                // reqwest clients are a handle on a shared pool, so the clone
                // keeps connection reuse across tasks.
                let client = client.clone();
                let semaphore = Arc::clone(&semaphore);
                tasks.spawn(async move {
                    // acquire_owned errors only when the semaphore is closed,
                    // and nothing here closes it.
                    let _permit = semaphore
                        .acquire_owned()
                        .await
                        .expect("semaphore is never closed");
                    let body = ptr.location.fetch(&client, &cache).await?;
                    parse_urlset(&ptr.location, &body)
                });
            }
            tasks.join_all().await.into_iter().collect()
        }
        Parsed::UrlSet(urlset) => Ok(vec![urlset]),
    }
}

/// Fetch the sitemap at `args.url`, following one level of sitemap-index
/// nesting, and print the deduplicated, sorted URLs to stdout.
///
/// # Errors
///
/// Returns an error when any sitemap cannot be fetched or parsed, or when
/// writing to stdout fails for any reason other than a closed pipe.
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);
    let cache = cache::load_cache(&args, CachePath::Default).unwrap_or_default();
    let cache = Arc::new(cache);

    let client = fetcher::build_client()?;
    let urlsets = get_urlsets(&args.url, &client, &cache).await?;

    cache::store_cache(&cache, args.no_cache, CachePath::Default);

    let urls = collect_urls(&urlsets);
    // An empty sitemap parses cleanly, so without this a zero-URL run is
    // indistinguishable from success with output swallowed somewhere.
    if urls.is_empty() {
        warn!(url=%args.url, "the sitemap contained no URLs");
    }

    // Stdout is line-buffered, so a large sitemap would otherwise pay one
    // write syscall per URL; the BufWriter batches them.
    write_urls(BufWriter::new(io::stdout().lock()), &urls)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use hegel::generators;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::cache::Cache;
    use crate::error::Error;
    use crate::sitemap::{SitemapUrl, UrlSet};

    use super::{collect_urls, get_urlsets, write_urls};
    use crate::fetcher::build_client;
    use std::sync::Arc;
    use url::Url;

    // A writer that always fails with the given kind, standing in for a stdout
    // that has gone away.
    struct FailingWriter(std::io::ErrorKind);

    impl std::io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::new(self.0, "stub failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_urls_writes_one_per_line() {
        let mut buf = Vec::new();
        write_urls(
            &mut buf,
            &[
                "https://a.example/".to_string(),
                "https://b.example/".to_string(),
            ],
        )
        .unwrap();
        assert_eq!(buf, b"https://a.example/\nhttps://b.example/\n");
    }

    #[test]
    fn write_urls_treats_broken_pipe_as_success() {
        let urls = vec!["https://a.example/".to_string()];
        assert!(write_urls(FailingWriter(std::io::ErrorKind::BrokenPipe), &urls).is_ok());
    }

    #[test]
    fn write_urls_propagates_other_io_errors() {
        let urls = vec!["https://a.example/".to_string()];
        assert!(matches!(
            write_urls(FailingWriter(std::io::ErrorKind::PermissionDenied), &urls),
            Err(Error::Io(_))
        ));
    }

    fn urlset_from(urls: &[Url]) -> UrlSet {
        UrlSet {
            urls: urls
                .iter()
                .map(|location| SitemapUrl {
                    location: location.clone(),
                })
                .collect(),
        }
    }

    // A vec of well-formed URLs, the unit `collect_urls` operates on.
    #[hegel::composite]
    fn url_vec(tc: hegel::TestCase) -> Vec<Url> {
        let strings = tc.draw(generators::vecs(
            generators::from_regex(r"https?://[a-z]{1,8}\.[a-z]{2,4}/[a-z]{0,10}").fullmatch(true),
        ));
        strings
            .into_iter()
            .map(|s| Url::parse(&s).expect("generated string is a valid URL"))
            .collect()
    }

    // The output is exactly the sorted, deduplicated set of input locations. A
    // BTreeSet is the independent oracle for "sorted + unique".
    #[hegel::test]
    fn collect_urls_is_sorted_and_deduped(tc: hegel::TestCase) {
        let urls = tc.draw(url_vec());
        let got = collect_urls(&[urlset_from(&urls)]);
        let oracle: Vec<String> = urls
            .iter()
            .map(ToString::to_string)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        assert_eq!(got, oracle);
    }

    // Adding more url sets never drops a URL that was already present.
    #[hegel::test]
    fn collect_urls_union_is_superset(tc: hegel::TestCase) {
        let a = tc.draw(url_vec());
        let b = tc.draw(url_vec());
        let only_a = collect_urls(&[urlset_from(&a)]);
        let union = collect_urls(&[urlset_from(&a), urlset_from(&b)]);
        for url in only_a {
            assert!(union.contains(&url));
        }
    }

    fn index_xml(child_locs: &[String]) -> String {
        let mut entries = String::new();
        for loc in child_locs {
            entries.push_str("<sitemap><loc>");
            entries.push_str(loc);
            entries.push_str("</loc></sitemap>");
        }
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{entries}</sitemapindex>"#
        )
    }

    fn urlset_xml(locs: &[&str]) -> String {
        let mut entries = String::new();
        for &loc in locs {
            entries.push_str("<url><loc>");
            entries.push_str(loc);
            entries.push_str("</loc></url>");
        }
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{entries}</urlset>"#
        )
    }

    // The flagship sociable test: a sitemap index fans out to two child url sets;
    // the result is their sorted, deduplicated union.
    #[tokio::test]
    async fn index_fans_out_and_dedups_union() {
        let server = MockServer::start().await;
        let a_url = format!("{}/a.xml", server.uri());
        let b_url = format!("{}/b.xml", server.uri());

        Mock::given(method("GET"))
            .and(path("/sitemap-index.xml"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(index_xml(&[a_url.clone(), b_url.clone()])),
            )
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/a.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(urlset_xml(&[
                "https://example.com/a",
                "https://example.com/b",
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/b.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(urlset_xml(&[
                "https://example.com/b",
                "https://example.com/c",
            ])))
            .mount(&server)
            .await;

        let index_url = Url::parse(&format!("{}/sitemap-index.xml", server.uri())).unwrap();
        let cache = Arc::new(Cache::new());
        let urlsets = get_urlsets(&index_url, &build_client().unwrap(), &cache)
            .await
            .expect("fetched index and children");
        let urls = collect_urls(&urlsets);

        assert_eq!(
            urls,
            vec![
                "https://example.com/a".to_string(),
                "https://example.com/b".to_string(),
                "https://example.com/c".to_string(),
            ]
        );
    }

    // A plain (non-index) url set is collected directly, sorted and deduplicated.
    #[tokio::test]
    async fn single_urlset_is_collected_sorted_and_deduped() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/sitemap.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(urlset_xml(&[
                "https://example.com/y",
                "https://example.com/x",
                "https://example.com/x",
            ])))
            .mount(&server)
            .await;

        let url = Url::parse(&format!("{}/sitemap.xml", server.uri())).unwrap();
        let cache = Arc::new(Cache::new());
        let urlsets = get_urlsets(&url, &build_client().unwrap(), &cache)
            .await
            .expect("fetched url set");
        let urls = collect_urls(&urlsets);

        assert_eq!(
            urls,
            vec![
                "https://example.com/x".to_string(),
                "https://example.com/y".to_string(),
            ]
        );
    }
}
