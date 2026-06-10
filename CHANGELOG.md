# Changelog

## 0.1.8

### Patch Changes

- [`719fd27`](https://github.com/lukehsiao/sitemap2urllist/commit/719fd2733e884066920b8f231918580a2c459bd5) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Treat cache persistence as best-effort: a failed write warns instead of failing the run.

- [`9c70f32`](https://github.com/lukehsiao/sitemap2urllist/commit/9c70f3259eac8b86d3f0133c88b7d7e92ebcb5c6) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Stop caching empty bodies as servable content, and only send conditional requests when the cache can actually serve a 304.

- [`d2bc060`](https://github.com/lukehsiao/sitemap2urllist/commit/d2bc06083580dbf97d38101a601e484a47d4d3f3) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Bound concurrent child fetches of a sitemap index to 32.

- [`3168756`](https://github.com/lukehsiao/sitemap2urllist/commit/3168756fd41784dc43d8b80c23eaf6fd7a7bff76) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Defer cache-file truncation until the exclusive lock is held, and flush the writer so write errors surface.

- [`871dfdc`](https://github.com/lukehsiao/sitemap2urllist/commit/871dfdc0adb690961ed9083c6d63343ef636f201) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Never panic when a cached 429 retry window overflows the representable time range.

- [`f398803`](https://github.com/lukehsiao/sitemap2urllist/commit/f39880303d09401cbc24ff315081aecfbc888f2d) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Replace the 30s total request deadline with granular timeouts: 10s connect, 30s stall, 5 minute ceiling.

- [`db742e9`](https://github.com/lukehsiao/sitemap2urllist/commit/db742e96714a176b98111b74069cd8e67740b071) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Support gzip-compressed sitemap files (`sitemap.xml.gz`): bodies carrying the gzip magic bytes are decompressed, with the decompressed size held to the same 64 MiB cap so a compression bomb cannot bypass it.

- [`a928416`](https://github.com/lukehsiao/sitemap2urllist/commit/a9284162caeade12295273cc76747acb89db669a) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Tolerate junk in unused sitemap fields (e.g. empty `<priority>`), accept empty url sets, and reject non-sitemap documents like HTML error pages explicitly.

- [`aa5c362`](https://github.com/lukehsiao/sitemap2urllist/commit/aa5c3624d7803478bfb20e086a8d7b344823735c) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Decompress every member of a multi-member gzip sitemap instead of silently truncating it to the first member.

- [`bd8afc6`](https://github.com/lukehsiao/sitemap2urllist/commit/bd8afc6c13fb5819a34088b2d6e11226ba8e13d0) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Warn when a sitemap URL redirects.

- [`bb2faad`](https://github.com/lukehsiao/sitemap2urllist/commit/bb2faad0bf2f11ed1b0b9e8c9328e54b653c9b84) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Cap response bodies at 64 MiB.

- [`0cd1132`](https://github.com/lukehsiao/sitemap2urllist/commit/0cd1132744be439b787a0ee575a1c1a3fe28839a) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Accept the HTTP-date form of `Retry-After` in addition to delta-seconds.

- [`bb34881`](https://github.com/lukehsiao/sitemap2urllist/commit/bb348816cbdabc22bcd8191bc095ff9a26dc6985) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Adopt rotated `ETag`/`Last-Modified` validators from 304 responses.

- [`d678e56`](https://github.com/lukehsiao/sitemap2urllist/commit/d678e561ac44167a2dccba081de32f4fb2d074d4) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Share one HTTP client across all fetches in a run for connection reuse.

- [`f53916f`](https://github.com/lukehsiao/sitemap2urllist/commit/f53916f87c0dd286abc1c35ae53405c69cb4675b) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Exit quietly when stdout closes early (e.g. piping into `head`) instead of panicking.

- [`171f442`](https://github.com/lukehsiao/sitemap2urllist/commit/171f44223c77582d2e81b8011b19f57443e42d75) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Warn when a sitemap run produces zero URLs, so an empty sitemap is distinguishable from output going missing.

- [`0b66fb6`](https://github.com/lukehsiao/sitemap2urllist/commit/0b66fb6e31a5dd218ae91e4db2f34354eea25c19) Thanks [@lukehsiao](https://github.com/lukehsiao)! - Show warnings by default; `-q` restores errors-only.

<pre>
$ git-stats v0.1.7..v0.1.8
Author      Commits  Changed Files  Insertions  Deletions  Net Δ
Luke Hsiao       22             49       +1371       -160  +1211
Total            22             49       +1371       -160  +1211
</pre>

## 0.1.7

### Patch Changes

- [`e69c13b`](https://github.com/lukehsiao/sitemap2urllist/commit/e69c13ba04738eb317b4e1a7a4233e7d44c88dcb) Thanks [@lukehsiao](https://github.com/lukehsiao)! - **refactor**: now sorts the output URLs for deterministic ordering.

<pre>
$ git-stats v0.1.6..v0.1.7
Author               Commits  Changed Files  Insertions  Deletions  Net Δ
Luke Hsiao                 4             21       +2132       -628  +1504
github-actions[bot]        1              5        +222        -11   +211
dependabot[bot]            1              1          +1         -1      0
Total                      6             27       +2355       -640  +1715
</pre>

## 0.1.6

### Patch Changes

- [`6fbc4ad`](https://github.com/lukehsiao/sitemap2urllist/commit/6fbc4ade7354bbaedabede99b6cdc5d7515cc4bf) Thanks [@lukehsiao](https://github.com/lukehsiao)! - **Documentation**: fix example usage in README.

<pre>
$ git-stats v0.1.5..v0.1.6
Author      Commits  Changed Files  Insertions  Deletions  Net Δ
Luke Hsiao        1              2          +6         -1     +5
Total             1              2          +6         -1     +5
</pre>

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
