use anyhow::{Context, Result};
use std::collections::HashSet;

use super::fetcher::{PageFetcher, Html};

pub struct WikiScraper<F: PageFetcher> {
    fetcher: F,
}

impl<F: PageFetcher> WikiScraper<F> {
    pub fn with_fetcher(fetcher: F) -> Self {
        Self { fetcher }
    }

    pub fn get_wiki_links(&self, page_name: &str) -> Result<HashSet<String>> {
        let html = self.get_page_contents(page_name)?;
        self.parse_wiki_links(&html)
    }

    #[inline]
    fn get_page_contents(&self, page_name: &str) -> Result<Html> {
        self.fetcher
            .fetch(page_name)
            .context(format!("Failed to fetch page {}", page_name))
    }

    fn parse_wiki_links(&self, html: &Html) -> Result<HashSet<String>> {
        let dom = tl::parse(&html.0, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let anchors = dom
            .query_selector("a[href]")
            .context("failed to parse query selector")?;

        let links = anchors
            .flat_map(|anchor| {
                let anchor = anchor
                    .get(parser)
                    .expect("failed to resolve node")
                    .as_tag()
                    .expect("Failed to cast to HTMLTag");

                let attributes = anchor.attributes();
                let href = attributes
                    .get("href")
                    .context("attribute not found or malformed")
                    .unwrap_or_default();
                let link = href
                    .map(|bytes| bytes.try_as_utf8_str().unwrap_or_default().trim())
                    .unwrap_or_default();

                if link.starts_with("/wiki/") && !link.contains(":") {
                    Some(link.replace("/wiki/", ""))
                } else {
                    None
                }
            })
            .collect();

        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPageFetcher(String);
    impl PageFetcher for MockPageFetcher {
        fn fetch(&self, _page_name: &str) -> Result<Html> {
            Ok(Html(self.0.clone()))
        }
    }

    #[test]
    fn returns_expected_wiki_links() {
        let html = r#"
      <p>
      In <a href="/wiki/Topology​">topology</a>, the <b>long line</b> (or
      <b>Alexandroff line</b>) is a
      <a href="/wiki/Topological_space​">topological space</a> somewhat similar to
      the <a href="/wiki/Real_line​">real line</a>, but in a certain way "longer". It
      behaves locally just like the real line, but has different large-scale
      properties (e.g., it is neither
      <a href="/wiki/Lindel%C3%B6f_space​">Lindelöf</a> nor
      <a href="/wiki/Separable_space​">separable</a>). Therefore, it serves as one of
      the basic counterexamples of topology
      <a href="http://www.ams.org/mathscinet-getitem?mr=507446">[1]</a>.
      Intuitively, the usual real-number line consists of a countable number of line
      segments [0,1) laid end-to-end, whereas the long line is constructed from an
      uncountable number of such segments. You can consult
      <a href="/wiki/Special:BookSources/978-1-55608-010-4">this</a> book for more
      information.
      </p>
      "#;
        let scraper = WikiScraper::with_fetcher(MockPageFetcher(html.to_string()));

        let expected_links = vec![
            "Topology​".into(),
            "Topological_space​".into(),
            "Real_line​".into(),
            "Lindel%C3%B6f_space​".into(),
            "Separable_space​".into(),
        ];
        let links = scraper.get_wiki_links("page_name").unwrap();
        assert_eq!(links, HashSet::from_iter(expected_links));
    }
}
