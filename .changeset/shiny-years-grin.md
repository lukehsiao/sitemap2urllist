---
"sitemap2urllist": patch
---

**Feature**: we now cache by default to standard cache directories.
In addition, this cache is now stored as JSON, not CSV.

- **Linux**: `$XDG_CACHE_HOME/sitemap2urllist/cache.json` or `$HOME/.cache/sitemap2urllist/cache.json`
- **macOS**: `$HOME/Library/Caches/dev.hsiao.sitemap2urllist/cache.json`
- **Windows**: `{FOLDERID_LocalAppData}\hsiao\sitemap2urllist\cache\cache.json`
