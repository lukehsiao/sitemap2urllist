---
"sitemap2urllist": patch
---

Support gzip-compressed sitemap files (`sitemap.xml.gz`): bodies carrying the gzip magic bytes are decompressed, with the decompressed size held to the same 64 MiB cap so a compression bomb cannot bypass it.
