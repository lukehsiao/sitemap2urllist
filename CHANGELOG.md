# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

---
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
