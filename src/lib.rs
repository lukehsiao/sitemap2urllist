pub mod args;
pub mod cache;
pub mod sitemap;

use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use clap::{crate_name, crate_version};
use jiff::{Timestamp, ToSpan};
use quick_xml::de;
use reqwest::{Client, ClientBuilder, StatusCode};
use tokio::task::JoinSet;
use tracing::{debug, warn};
use url::Url;

use crate::{
    args::Args,
    cache::{Cache, CacheValue, StoreExt, SITEMAP_CACHE_FILE},
    sitemap::{SitemapIndex, UrlSet},
};

#[allow(clippy::too_many_lines)]
async fn fetch_with_cache(client: &Client, url: &Url, cache: &Arc<Cache>) -> Result<String> {
    let r = {
        let cache_value = cache.get(url);

        // Respect Retry-After Header if set in cache
        if let Some(ref cv) = cache_value {
            if let Some(retry) = cv.retry_after {
                if cv.timestamp + retry > Timestamp::now() {
                    debug!(timestamp=%cv.timestamp, retry_after=%retry, "skipping request due to 429, using response from cache");
                    return cv
                        .body
                        .clone()
                        .ok_or(anyhow!("Cache has an empty response for {}", url.as_str()));
                }
            }
        }

        // Otherwise, go fetch again
        let mut r = client.get(url.as_str());
        // Add friendly headers if cache is available
        if let Some(ref cv) = cache_value {
            if let Some(last_modified) = &cv.last_modified {
                r = r.header("If-Modified-Since", last_modified);
            }
            if let Some(etag) = &cv.etag {
                r = r.header("If-None-Match", etag);
            }
        }
        r
    };
    debug!(url=%url.as_str(), request=?r, "sending request");
    let body = match r.send().await {
        Ok(r) => {
            debug!(url=%url.as_str(), response=?r, "received response");
            match r.status() {
                s if s.is_success() || s == StatusCode::NOT_MODIFIED => {
                    // ETag values must have the actual quotes
                    let etag = r.headers().get("etag").and_then(|etag_value| {
                        // Convert header to str
                        etag_value.to_str().ok().map(|etag_str| {
                            if (etag_str.starts_with('"') && etag_str.ends_with('"'))
                                || (etag_str.starts_with("W/\"") && etag_str.ends_with('"'))
                            {
                                etag_str.to_string()
                            } else {
                                format!("\"{etag_str}\"")
                            }
                        })
                    });
                    let last_modified = r.headers().get("last-modified").and_then(|lm_value| {
                        lm_value.to_str().ok().map(std::string::ToString::to_string)
                    });
                    let status = r.status();
                    let mut body = r.text().await.ok();

                    // Update cache
                    {
                        let cache_value = cache.get_mut(url);
                        if let Some(mut cv) = cache_value {
                            if status == StatusCode::NOT_MODIFIED {
                                debug!(url=%url.as_str(), status=status.as_str(), "got 304, using sitemap from cache");
                                body.clone_from(&cv.body);
                            } else {
                                debug!(url=%url.as_str(), status=status.as_str(), "cache hit, using sitemap from body");
                                cv.etag = etag;
                                cv.last_modified = last_modified;
                                cv.body.clone_from(&body);
                            }
                            cv.timestamp = Timestamp::now();
                        } else {
                            debug!(url=%url.as_str(), status=status.as_str(), "using sitemap from body and adding to cache");
                            cache.insert(
                                url.clone(),
                                CacheValue {
                                    timestamp: Timestamp::now(),
                                    retry_after: None,
                                    etag,
                                    last_modified,
                                    body: body.clone(),
                                },
                            );
                        }
                    }
                    body.ok_or(anyhow!("sitemap is empty: {}", url.as_str()))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    let cache_value = cache.get_mut(url);
                    if let Some(mut cv) = cache_value {
                        cv.timestamp = Timestamp::now();
                        // Default to waiting 4 hrs if no Retry-After
                        let retry_after = r
                            .headers()
                            .get("retry-after")
                            .and_then(|retry_value| {
                                retry_value.to_str().ok().map(|retry_str| {
                                    retry_str.parse::<i64>().map(ToSpan::seconds).ok()
                                })
                            })
                            .unwrap_or(Some(4.hours()));
                        debug!(url=%url.as_str(), response=?r, "got 429, using sitemap from cache");
                        cv.timestamp = Timestamp::now();
                        cv.retry_after = retry_after;
                        cv.body
                            .clone()
                            .ok_or(anyhow!("sitemap is empty: {}", url.as_str()))
                    } else {
                        Err(anyhow!("rate limit error {}", url.as_str()))
                    }
                }
                unexpected => Err(anyhow!(
                    "unexpected error: {} for {}",
                    unexpected.as_str(),
                    url.as_str()
                )),
            }
        }
        Err(e) => {
            warn!(url=%url.as_str(), error=%e, "failed to get sitemap.");
            Err(e.into())
        }
    };
    body
}

async fn get_urlsets(url: &Url, cache: &Arc<Cache>) -> Result<Vec<UrlSet>> {
    // Get the main sitemap and start parsing
    let client = ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .user_agent(concat!(crate_name!(), '/', crate_version!()))
        .build()?;
    // let res = client.get(url).send().await?.text().await?;
    let res = fetch_with_cache(&client, url, cache).await?;

    // Google enforces that sitemap indexes cannot contain other sitemap indices,
    // so we don't go deeper than 1 level.
    let urlsets: Result<Vec<UrlSet>> = {
        let mut tasks = JoinSet::new();
        if let Ok(sitemap_idx) = de::from_str::<SitemapIndex>(&res) {
            for ptr in sitemap_idx.sitemaps {
                let client = client.clone();
                let cache = cache.clone();
                tasks.spawn(async move {
                    let url_set = de::from_str::<UrlSet>(
                        &fetch_with_cache(&client, &ptr.location, &cache).await?,
                    )?;
                    Ok(url_set)
                });
            }
            tasks.join_all().await.into_iter().collect()
        } else {
            Ok(vec![de::from_str::<UrlSet>(&res)?])
        }
    };
    urlsets
}

#[allow(clippy::missing_panics_doc)]
#[allow(clippy::missing_errors_doc)]
pub async fn run(args: Args) -> Result<()> {
    debug!(?args);
    let cache = cache::load_cache(&args).unwrap_or_default();
    let cache = Arc::new(cache);

    let urlsets = get_urlsets(&args.url, &cache).await?;

    for urlset in urlsets {
        for url in urlset.urls {
            println!("{}", url.location.as_str());
        }
    }

    if args.cache {
        cache.store(SITEMAP_CACHE_FILE)?;
    }

    Ok(())
}
