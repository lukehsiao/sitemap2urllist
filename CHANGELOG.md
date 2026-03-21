# Changelog

## 0.1.5

### Patch Changes

- [`c85a3cc`](https://github.com/lukehsiao/sitemap2urllist/commit/c85a3cc7f158e570ca014fbcd575d135df423b2d) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Add metadata and binaries [`cargo-binstall`](https://github.com/cargo-bins/cargo-binstall) compatibility.

- [`350b117`](https://github.com/lukehsiao/sitemap2urllist/commit/350b11735b688573989542c3e52c95ad44dd2ed9) Thanks [@lukehsiao](https://github.com/lukehsiao)! - **Feature**: we now cache by default to standard cache directories.
  In addition, this cache is now stored as JSON, not CSV.

  - **Linux**: `$XDG_CACHE_HOME/sitemap2urllist/cache.json` or `$HOME/.cache/sitemap2urllist/cache.json`
  - **macOS**: `$HOME/Library/Caches/dev.hsiao.sitemap2urllist/cache.json`
  - **Windows**: `{FOLDERID_LocalAppData}\hsiao\sitemap2urllist\cache\cache.json`

<pre>
$ git-stats v0.1.4..v0.1.5
Author           Commits  Changed Files  Insertions  Deletions  Net Δ
Luke Hsiao            12             45       +3208      -1575  +1633
dependabot[bot]        5             10         +25        -25      0
Total                 17             55       +3233      -1600  +1633
</pre>

## [0.1.4](https://github.com/lukehsiao/sitemap2urllist/compare/v0.1.3..v0.1.4) - 2025-01-02

### Documentation

- fix name of program in --help - ([cc8d1cd](https://github.com/lukehsiao/sitemap2urllist/commit/cc8d1cdb31632d44d2778fbe865dace00febf68b)) - Luke Hsiao

---

## [0.1.3](https://github.com/lukehsiao/sitemap2urllist/compare/v0.1.2..v0.1.3) - 2025-01-02

### Bug Fixes

- provide a more useful error message on invalid xml - ([ad9a500](https://github.com/lukehsiao/sitemap2urllist/commit/ad9a5001b6bd2f67f7252a64d13c84e27e116bfa)) - Luke Hsiao

---

## [0.1.2](https://github.com/lukehsiao/sitemap2urllist/compare/v0.1.1..v0.1.2) - 2025-01-01

### Features

- deduplicate the final list of urls - ([b1cde8f](https://github.com/lukehsiao/sitemap2urllist/commit/b1cde8f97b5cee52f80a78a2a72daa12963888cb)) - Luke Hsiao

### Documentation

- **(README)** add related tools section - ([2f6a19a](https://github.com/lukehsiao/sitemap2urllist/commit/2f6a19a1b684cc4296c437476083a9a903d9a93a)) - Luke Hsiao

---

## [0.1.1](https://github.com/lukehsiao/sitemap2urllist/compare/v0.1.0..v0.1.1) - 2025-01-01

Minor patch release just tweaking some wording in comments/README.
No code change.

---

## [0.1.0] - 2025-01-01

This is the initial release for `sitemap2urllist`.

This is a simple CLI tool which takes a URL to a sitemap (or sitemap index), and prints a list of the final URLs to stdout.

We also have a `--cache` option, which is highly recommended to reduce wasteful requests.
