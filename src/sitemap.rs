use quick_xml::{de, events::Event};
use serde::Deserialize;
use serde::de::Error as _;
use url::Url;

use crate::error::{Error, Result};

/// A parsed sitemap document, dispatched on its root element.
pub(crate) enum Parsed {
    Index(SitemapIndex),
    UrlSet(UrlSet),
}

/// The local name of the document's root element, if it has one. Namespace
/// prefixes are stripped, so `<ns:urlset>` reads as `urlset`.
fn root_element_name(xml: &str) -> Option<String> {
    let mut reader = quick_xml::Reader::from_str(xml);
    loop {
        match reader.read_event() {
            Ok(Event::Start(start) | Event::Empty(start)) => {
                return Some(String::from_utf8_lossy(start.local_name().as_ref()).into_owned());
            }
            Ok(Event::Eof) | Err(_) => return None,
            _ => {}
        }
    }
}

/// Error unless the document's root element is `expected`. The entry vectors
/// default to empty, so without this check almost any XML (say, an HTML error
/// page served with a 200) would deserialize as an empty sitemap and silently
/// print nothing; the root name is what tells them apart.
fn expect_root(url: &Url, xml: &str, expected: &str) -> Result<()> {
    match root_element_name(xml) {
        Some(name) if name == expected => Ok(()),
        Some(name) => Err(Error::InvalidXml {
            url: url.as_str().to_string(),
            source: quick_xml::DeError::custom(format!(
                "unexpected root element <{name}>, expected <{expected}>"
            )),
        }),
        None => Err(Error::InvalidXml {
            url: url.as_str().to_string(),
            source: quick_xml::DeError::custom("document has no root element"),
        }),
    }
}

/// Parse a document strictly as a `<urlset>`, attributing any failure to `url`.
/// Used for the children of a sitemap index, which must be url sets (Google does
/// not allow a sitemap index to nest another index).
pub(crate) fn parse_urlset(url: &Url, xml: &str) -> Result<UrlSet> {
    expect_root(url, xml, "urlset")?;
    de::from_str::<UrlSet>(xml).map_err(|source| Error::InvalidXml {
        url: url.as_str().to_string(),
        source,
    })
}

/// Parse a sitemap document, dispatching on the root element: a `<sitemapindex>`
/// becomes [`Parsed::Index`], anything else is parsed as a `<urlset>`. This never
/// panics; malformed input yields [`Error::InvalidXml`].
pub(crate) fn parse_sitemap(url: &Url, xml: &str) -> Result<Parsed> {
    if root_element_name(xml).as_deref() == Some("sitemapindex") {
        return de::from_str::<SitemapIndex>(xml)
            .map(Parsed::Index)
            .map_err(|source| Error::InvalidXml {
                url: url.as_str().to_string(),
                source,
            });
    }
    parse_urlset(url, xml).map(Parsed::UrlSet)
}

// Only <loc> is deserialized from sitemap entries. The optional fields the
// protocol defines (lastmod, changefreq, priority) are unused here, and
// leaving them out of the model means junk inside them (an empty <priority>,
// a malformed date) cannot fail a parse.

#[derive(Debug, Deserialize)]
#[serde(rename = "sitemapindex", rename_all = "lowercase")]
pub(crate) struct SitemapIndex {
    #[serde(rename = "sitemap", default)]
    pub(crate) sitemaps: Vec<SitemapPtr>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SitemapPtr {
    #[serde(rename = "loc")]
    pub(crate) location: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "urlset", rename_all = "lowercase")]
pub(crate) struct UrlSet {
    #[serde(rename = "url", default)]
    pub(crate) urls: Vec<SitemapUrl>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SitemapUrl {
    #[serde(rename = "loc")]
    pub(crate) location: Url,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use quick_xml::de;

    use hegel::generators;

    use super::*;
    use crate::error::Result;

    const SITEMAP_INDEX: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
   <sitemap>
      <loc>http://www.example.com/sitemap1.xml.gz</loc>
      <lastmod>2004-10-01T18:23:17+00:00</lastmod>
   </sitemap>
   <sitemap>
      <loc>http://www.example.com/sitemap2.xml.gz</loc>
      <lastmod>2005-01-01</lastmod>
   </sitemap>
</sitemapindex> 
    "#;

    const EXAMPLE_SITEMAP: &str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
   <url>
      <loc>http://www.example.com/</loc>
      <lastmod>2005-01-01</lastmod>
      <changefreq>monthly</changefreq>
      <priority>0.8</priority>
   </url>
   <url>
      <loc>http://www.example.com/catalog?item=12&amp;desc=vacation_hawaii</loc>
      <changefreq>weekly</changefreq>
   </url>
   <url>
      <loc>http://www.example.com/catalog?item=73&amp;desc=vacation_new_zealand</loc>
      <lastmod>2004-12-23</lastmod>
      <changefreq>weekly</changefreq>
   </url>
   <url>
      <loc>http://www.example.com/catalog?item=74&amp;desc=vacation_newfoundland</loc>
      <lastmod>2004-12-23T18:00:15+00:00</lastmod>
      <priority>0.3</priority>
   </url>
   <url>
      <loc>http://www.example.com/catalog?item=83&amp;desc=vacation_usa</loc>
      <lastmod>2004-11-23</lastmod>
   </url>
</urlset>
    "#;

    #[test]
    fn nonnested_deserialization_works() -> Result<()> {
        let sitemap: UrlSet = de::from_str(EXAMPLE_SITEMAP)?;
        dbg!("{:?}", &sitemap);
        assert_eq!(sitemap.urls.len(), 5);
        Ok(())
    }

    #[test]
    fn nonnested_deserialization_parses_urls() -> Result<()> {
        let sitemap: UrlSet = de::from_str(EXAMPLE_SITEMAP)?;
        dbg!("{:?}", &sitemap);
        assert_eq!(sitemap.urls.len(), 5);
        assert_eq!(
            sitemap.urls[0].location,
            Url::parse("http://www.example.com/")?
        );
        assert_eq!(
            sitemap.urls[1].location,
            Url::parse("http://www.example.com/catalog?item=12&desc=vacation_hawaii")?
        );
        Ok(())
    }

    #[test]
    fn nested_deserialization_works() -> Result<()> {
        let sitemap_idx: SitemapIndex = de::from_str(SITEMAP_INDEX)?;
        dbg!("{:?}", &sitemap_idx);
        assert_eq!(sitemap_idx.sitemaps.len(), 2);
        Ok(())
    }

    #[test]
    fn nested_deserialization_parses_urls() -> Result<()> {
        let sitemap_idx: SitemapIndex = de::from_str(SITEMAP_INDEX)?;
        dbg!("{:?}", &sitemap_idx);
        assert_eq!(sitemap_idx.sitemaps.len(), 2);
        assert_eq!(
            sitemap_idx.sitemaps[0].location,
            Url::parse("http://www.example.com/sitemap1.xml.gz")?
        );
        assert_eq!(
            sitemap_idx.sitemaps[1].location,
            Url::parse("http://www.example.com/sitemap2.xml.gz")?
        );
        Ok(())
    }

    // `parse_sitemap` dispatches on the root element.
    #[test]
    fn parse_sitemap_dispatches_on_root() -> Result<()> {
        let url = Url::parse("https://example.com/")?;
        assert!(matches!(
            parse_sitemap(&url, SITEMAP_INDEX)?,
            Parsed::Index(_)
        ));
        assert!(matches!(
            parse_sitemap(&url, EXAMPLE_SITEMAP)?,
            Parsed::UrlSet(_)
        ));
        Ok(())
    }

    // `parse_sitemap` returns an error (never panics) on arbitrary input.
    #[hegel::test]
    fn parse_sitemap_never_panics(tc: hegel::TestCase) {
        let xml = tc.draw(generators::text());
        let url = Url::parse("https://example.com/sitemap.xml").expect("valid url");
        let _ = parse_sitemap(&url, &xml);
    }

    // Rendering a urlset and parsing it back yields exactly the input
    // locations, including URLs whose query strings need XML escaping.
    #[hegel::test]
    fn urlset_round_trips_locations(tc: hegel::TestCase) {
        let urls: Vec<Url> = tc
            .draw(generators::vecs(
                generators::from_regex(
                    r"https?://[a-z]{1,8}\.[a-z]{2,4}/[a-z]{0,8}(\?[a-z]{1,3}=[a-z]{1,3}(&[a-z]{1,3}=[a-z]{1,3}){0,2})?",
                )
                .fullmatch(true),
            ))
            .into_iter()
            .map(|s| Url::parse(&s).expect("generated string is a valid URL"))
            .collect();
        let entries: String = urls.iter().fold(String::new(), |mut acc, u| {
            acc.push_str("<url><loc>");
            acc.push_str(&u.as_str().replace('&', "&amp;"));
            acc.push_str("</loc></url>");
            acc
        });
        let xml = format!(
            r#"<?xml version="1.0"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">{entries}</urlset>"#
        );
        let sitemap_url = Url::parse("https://example.com/sitemap.xml").expect("valid url");
        let parsed = parse_urlset(&sitemap_url, &xml).expect("rendered urlset parses");
        let got: Vec<&str> = parsed.urls.iter().map(|u| u.location.as_str()).collect();
        let want: Vec<&str> = urls.iter().map(Url::as_str).collect();
        assert_eq!(got, want);
    }

    // The optional per-URL fields (lastmod, changefreq, priority) are unused,
    // so junk inside them must not be able to fail the run. Sloppy generators
    // emit empty or non-numeric <priority> elements in the wild.
    #[test]
    fn junk_optional_fields_do_not_fail_parse() -> Result<()> {
        let url = Url::parse("https://example.com/sitemap.xml")?;
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
   <url>
      <loc>https://example.com/a</loc>
      <priority></priority>
   </url>
   <url>
      <loc>https://example.com/b</loc>
      <lastmod>not-a-date</lastmod>
      <priority>high</priority>
   </url>
</urlset>"#;
        let urlset = parse_urlset(&url, xml)?;
        assert_eq!(urlset.urls.len(), 2);
        Ok(())
    }

    // Generators emit empty url sets for brand-new sites. The XSD technically
    // requires at least one <url>, but rejecting an empty sitemap helps nobody:
    // zero URLs is the honest answer.
    #[test]
    fn empty_urlset_parses_to_zero_urls() -> Result<()> {
        let url = Url::parse("https://example.com/sitemap.xml")?;
        for xml in [
            r#"<?xml version="1.0"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"></urlset>"#,
            r#"<?xml version="1.0"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"/>"#,
        ] {
            let urlset = parse_urlset(&url, xml)?;
            assert!(urlset.urls.is_empty());
        }
        Ok(())
    }

    #[test]
    fn empty_sitemapindex_parses_to_zero_children() -> Result<()> {
        let url = Url::parse("https://example.com/sitemap.xml")?;
        let xml = r#"<?xml version="1.0"?><sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"></sitemapindex>"#;
        let Parsed::Index(index) = parse_sitemap(&url, xml)? else {
            panic!("expected Index");
        };
        assert!(index.sitemaps.is_empty());
        Ok(())
    }

    // Windows generators prepend a UTF-8 BOM; quick-xml tolerates it and the
    // root-name dispatch must too.
    #[test]
    fn bom_prefixed_sitemap_parses() -> Result<()> {
        let url = Url::parse("https://example.com/sitemap.xml")?;
        let xml = format!(
            "\u{feff}{}",
            r#"<?xml version="1.0"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://example.com/a</loc></url>
</urlset>"#
        );
        let urlset = parse_urlset(&url, &xml)?;
        assert_eq!(urlset.urls.len(), 1);
        Ok(())
    }

    // Pretty-printers put <loc> values on their own padded line; the url
    // crate strips the surrounding whitespace when parsing.
    #[test]
    fn whitespace_padded_loc_parses() -> Result<()> {
        let url = Url::parse("https://example.com/sitemap.xml")?;
        let xml = r#"<?xml version="1.0"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>
      https://example.com/padded
    </loc>
  </url>
</urlset>"#;
        let urlset = parse_urlset(&url, xml)?;
        assert_eq!(
            urlset.urls[0].location,
            Url::parse("https://example.com/padded")?
        );
        Ok(())
    }

    // quick-xml matches local names, so a namespace-prefixed sitemap parses
    // and the prefix-stripping in root_element_name agrees with it.
    #[test]
    fn namespace_prefixed_sitemap_parses() -> Result<()> {
        let url = Url::parse("https://example.com/sitemap.xml")?;
        let xml = r#"<?xml version="1.0"?>
<sm:urlset xmlns:sm="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sm:url><sm:loc>https://example.com/a</sm:loc></sm:url>
</sm:urlset>"#;
        let Parsed::UrlSet(urlset) = parse_sitemap(&url, xml)? else {
            panic!("expected UrlSet");
        };
        assert_eq!(urlset.urls.len(), 1);
        Ok(())
    }

    // An HTML error page served with a 200 must stay an error, not become an
    // empty sitemap that silently prints nothing.
    #[test]
    fn html_document_is_an_error() {
        let url = Url::parse("https://example.com/sitemap.xml").unwrap();
        let html = "<html><body><h1>404 Not Found</h1></body></html>";
        assert!(parse_sitemap(&url, html).is_err());
        assert!(parse_urlset(&url, html).is_err());
    }

    // A sitemap index nested as the child of another index is rejected by
    // parse_urlset (Google forbids nesting), not treated as an empty url set.
    #[test]
    fn parse_urlset_rejects_sitemapindex_root() {
        let url = Url::parse("https://example.com/sitemap.xml").unwrap();
        assert!(parse_urlset(&url, SITEMAP_INDEX).is_err());
    }

    #[test]
    fn empty_document_is_an_error() {
        let url = Url::parse("https://example.com/sitemap.xml").unwrap();
        assert!(parse_sitemap(&url, "").is_err());
        assert!(parse_urlset(&url, "").is_err());
    }
}
