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

## Usage

```
Read a sitemap and output a list of URLs.

Usage: sitemap2urllist [OPTIONS] <URL>

Arguments:
  <URL>  The URL to a sitemap

Options:
  -c, --cache                          Use request cache stored on disk at `.sitemapcache` (recommended)
      --max-cache-age <MAX_CACHE_AGE>  Discard all cached requests older than this duration [default: 14d]
  -v, --verbose...                     Increase logging verbosity
  -q, --quiet...                       Decrease logging verbosity
  -h, --help                           Print help (see more with '--help')
  -V, --version                        Print version
```

### Example Usage with Lychee

At some point, it is likely link checkers like lychee obviate the need for this tool by implementing [recursive link checking](https://github.com/lycheeverse/lychee/issues/78).

In the meantime, it is easy to run a link check from your local machine on an entire website as defined by its sitemap by doing something like the following.

```
sitemap2urllist https://www.numbersstation.ai/sitemap.xml --cache | xargs lychee --cache
```

Note you can combine this with [lychee's configuration](https://lychee.cli.rs/usage/config/) to do things like cache or ignore certain errors, etc.
