use std::{
    fs,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use dashmap::DashMap;
use directories::ProjectDirs;
use jiff::{Span, Timestamp, ToSpan};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use url::Url;

use crate::args::Args;

const MAX_SPAN_SEC: i64 = 631_107_417_600;

/// Options for loading cache
#[derive(Copy, Clone, Debug)]
pub(crate) enum CachePath<'a> {
    Default,
    #[allow(dead_code)]
    Path(&'a Path),
}

/// Describes a sitemap result that can be serialized to disk
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CacheValue {
    pub(crate) timestamp: Timestamp,
    pub(crate) retry_after: Option<Span>,
    pub(crate) last_modified: Option<String>,
    pub(crate) etag: Option<String>,
    pub(crate) body: Option<String>,
}

/// Get the path to cache location.
pub(crate) fn get_cache_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("dev", "hsiao", "sitemap2urllist") {
        return Some(proj_dirs.cache_dir().join("cache.json"));
    }
    None
}

pub(crate) type Cache = DashMap<Url, CacheValue>;

pub(crate) trait StoreExt {
    /// Store the cache under the given path. Update access timestamps
    fn store<T: AsRef<Path>>(&self, path: T) -> Result<()>;

    /// Load cache from path. Discard entries older than `max_age_secs`
    fn load<T: AsRef<Path>>(path: T, max_age_secs: u64, now: Timestamp) -> Result<Cache>;
}

impl StoreExt for Cache {
    fn store<T: AsRef<Path>>(&self, path: T) -> Result<()> {
        // Ensure the parent directory exists
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let f = fs::File::create(path)?;
        // Grab a lock to avoid multiple processes writing simultaneously
        f.lock()?;
        let w = BufWriter::new(f);
        serde_json::to_writer_pretty(w, self)?;
        Ok(())
    }

    fn load<T: AsRef<Path>>(path: T, max_age_secs: u64, now: Timestamp) -> Result<Cache> {
        let clamped_secs: i64 = max_age_secs.min(MAX_SPAN_SEC as u64).cast_signed();
        let f = fs::File::open(path)?;
        // Acquire a shared lock so multiple readers can coexist, but no writers
        f.lock_shared()?;
        let r = BufReader::new(f);
        let map: DashMap<Url, CacheValue> = serde_json::from_reader(r)?;

        // Remove entries older than `max_age_secs`
        let current_ts = now;
        let threshold = clamped_secs.seconds();
        let keys_to_remove: Vec<Url> = map
            .iter()
            .filter_map(|entry| {
                let v = entry.value();
                if (current_ts - v.timestamp).compare(threshold).ok()? == std::cmp::Ordering::Less {
                    None
                } else {
                    Some(entry.key().clone())
                }
            })
            .collect();
        for k in keys_to_remove {
            map.remove(&k);
        }

        Ok(map)
    }
}

/// Load cache (if exists and is still valid).
///
/// This returns an `Option` as starting without a cache is a common scenario
/// and we silently discard errors on purpose.
pub(crate) fn load_cache(args: &Args, cache_path: CachePath) -> Option<Cache> {
    if args.no_cache {
        return None;
    }
    let default_cache_path = get_cache_path();
    let cache_path = match cache_path {
        CachePath::Default if default_cache_path.is_none() => return None,
        CachePath::Default => default_cache_path.unwrap(),
        CachePath::Path(p) => p.to_path_buf(),
    };
    // Discard entire cache if it hasn't been updated since `max_cache_age`.
    // This is an optimization, which avoids iterating over the file and
    // checking the age of each entry.
    match fs::metadata(&cache_path) {
        Err(_e) => {
            // No cache found; silently start with empty cache
            return None;
        }
        Ok(metadata) => {
            let modified = metadata.modified().ok()?;
            let elapsed = modified.elapsed().ok()?;
            if elapsed > args.max_cache_age {
                warn!(
                    "Cache is too old (age: {:#?}, max age: {:#?}). Discarding and recreating.",
                    Duration::from_secs(elapsed.as_secs()),
                    Duration::from_secs(args.max_cache_age.as_secs())
                );
                return None;
            }
            info!(
                "Cache is recent (age: {:#?}, max age: {:#?}). Using.",
                Duration::from_secs(elapsed.as_secs()),
                Duration::from_secs(args.max_cache_age.as_secs())
            );
        }
    }

    let cache = Cache::load(cache_path, args.max_cache_age.as_secs(), Timestamp::now());
    match cache {
        Ok(cache) => Some(cache),
        Err(e) => {
            warn!("Error while loading cache: {e}. Continuing without.");
            None
        }
    }
}
