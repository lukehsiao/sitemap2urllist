---
"sitemap2urllist": patch
---

**fix**: reject certificates issued by the compromised eMudhra emSign G3 intermediates ([Mozilla bug 2083449](https://bugzilla.mozilla.org/show_bug.cgi?id=2083449)).

The private keys of eight eMudhra intermediates are now public, so anyone can mint certificates that chain to them. `rustls-webpki` 0.103.16 rejects any path through those keys ([RUSTSEC-2026-0382](https://rustsec.org/advisories/RUSTSEC-2026-0382)), which protects Linux builds even while the system trust store still holds the emSign roots. `webpki-root-certs` 1.0.10 drops `emSign ECC Root CA - G3` from the bundle that `wasm32` builds use ([RUSTSEC-2026-0383](https://rustsec.org/advisories/RUSTSEC-2026-0383)). macOS and Windows builds use the OS verifier, so they depend on OS trust updates.

This release also takes `rustls` 0.23.46, which fixes [RUSTSEC-2026-0346](https://rustsec.org/advisories/RUSTSEC-2026-0346), [RUSTSEC-2026-0347](https://rustsec.org/advisories/RUSTSEC-2026-0347), [RUSTSEC-2026-0348](https://rustsec.org/advisories/RUSTSEC-2026-0348), and [RUSTSEC-2026-0349](https://rustsec.org/advisories/RUSTSEC-2026-0349), and upgrades all other dependencies to their latest versions.
