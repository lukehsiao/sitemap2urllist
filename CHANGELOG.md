# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

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
