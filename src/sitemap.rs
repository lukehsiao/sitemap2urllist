use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
#[serde(rename = "sitemapindex", rename_all = "lowercase")]
pub(crate) struct SitemapIndex {
    #[serde(rename = "@xmlns")]
    pub(crate) _xmlns: String,
    #[serde(rename = "sitemap")]
    pub(crate) sitemaps: Vec<SitemapPtr>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SitemapPtr {
    #[serde(rename = "loc")]
    pub(crate) location: Url,
    #[serde(rename = "lastmod")]
    pub(crate) _last_modified: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "urlset", rename_all = "lowercase")]
pub(crate) struct UrlSet {
    #[serde(rename = "@xmlns")]
    pub(crate) _xmlns: String,
    #[serde(rename = "url")]
    pub(crate) urls: Vec<SitemapUrl>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct SitemapUrl {
    #[serde(rename = "loc")]
    pub(crate) location: Url,
    #[serde(rename = "lastmod")]
    pub(crate) _last_modified: Option<String>,
    #[serde(rename = "changefreq")]
    pub(crate) _change_frequency: Option<String>,
    #[serde(rename = "priority")]
    pub(crate) _priority: Option<f64>,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use quick_xml::de;

    use super::*;

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
}
