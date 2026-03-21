<h1 align="center">
    🌐<br>
    sitemap2urllist
</h1>
<div align="center">
    <strong>Read a sitemap and output a list of URLs.</strong>
</div>
<br>
<div align="center">
  <a href="https://github.com/lukehsiao/sitemap2urllist/actions/workflows/general.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/lukehsiao/sitemap2urllist/general.yml" alt="Build Status">
  </a>
  <a href="https://crates.io/crates/sitemap2urllist">
    <img src="https://img.shields.io/crates/v/sitemap2urllist" alt="Version">
  </a>
  <a href="https://github.com/lukehsiao/sitemap2urllist/blob/main/LICENSE.md">
    <img src="https://img.shields.io/crates/l/sitemap2urllist" alt="License">
  </a>
</div>
<br>

`sitemap2urllist` is a CLI tool for parsing a sitemap and outputting a simple list of URLs, which can easily be piped into other tools (e.g., [lychee](https://github.com/lycheeverse/lychee)).

## Install

```
cargo install --locked sitemap2urllist
```

Or, if you use [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall):

```
cargo binstall sitemap2urllist
```

## Usage

```
Read a sitemap and output a list of URLs.

Usage: sitemap2urllist [OPTIONS] <URL>

Arguments:
  <URL>  The URL to a sitemap

Options:
      --no-cache                       Do NOT use request cache stored on disk
      --max-cache-age <MAX_CACHE_AGE>  Discard all cached requests older than this duration [default: 30d]
  -v, --verbose...                     Increase logging verbosity
  -q, --quiet...                       Decrease logging verbosity
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

### Example Usage with Lychee

At some point, it is likely link checkers like lychee obviate the need for this tool by implementing [recursive link checking](https://github.com/lycheeverse/lychee/issues/78).

In the meantime, it is easy to run a link check from your local machine on an entire website as defined by its sitemap by doing something like the following.

```
sitemap2urllist https://alumni.cottonwoodhigh.school/sitemap-index.xml --cache | xargs lychee --cache
```

Note you can combine this with [lychee's configuration](https://lychee.cli.rs/usage/config/) to do things like cache or ignore certain errors, etc.

## Caching

We use OS-standard locations for caching.

- **Linux**: `$XDG_CACHE_HOME/sitemap2urllist/cache.json` or `$HOME/.cache/sitemap2urllist/cache.json`
- **macOS**: `$HOME/Library/Caches/dev.hsiao.sitemap2urllist/cache.json`
- **Windows**: `{FOLDERID_LocalAppData}\hsiao\sitemap2urllist\cache\cache.json`

The cache file is simple JSON.

The cache only prevents refetching a feed if the feed source responds with a 429.
In this case, we respect `Retry-After`, or default to 4 hours.
Otherwise, we use the cache to send conditional requests by respecting the `ETag` and `Last-Modified` headers.

## Related Tools

- [Sitemap-to-Urllist](https://github.com/matejkosiarcik/sitemap2urllist) (rust/shell/typescript): Simple sitemap.xml to urllist.txt converter (**abandoned**)
