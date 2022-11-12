use std::{collections::HashSet, error::Error};
use anyhow::{Result, Context};

pub struct WikiScraper {}

impl WikiScraper {
    pub fn new() -> WikiScraper {
        WikiScraper {}
    }

    pub fn scrape(&self, page_name: &str) -> Result<String, Box<dyn Error>> {
        unimplemented!("Scrape the url: {}", page_name);
    }

    fn get_page_contents(&self, page_name: &str) -> Result<String, Box<dyn Error>> {
        unimplemented!("Get the page with name {} and return HTML", page_name);
    }

    fn get_wiki_links(&self, html: &str) -> Result<HashSet<String>, Box<dyn Error>> {
        let dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let anchors = dom
            .query_selector("a[href]")
            .expect("failed to parse query selector");

        Ok(anchors
            .flat_map(|anchor| {
                let anchor = anchor
                    .get(parser)
                    .expect("failed to resolve node")
                    .as_tag()
                    .expect("Failed to cast to HTMLTag");

                let attributes = anchor.attributes();
                let href = attributes
                    .get("href")
                    .expect("attribute not found or malformed");
                let link = href.map(|bytes| {
                    bytes
                        .try_as_utf8_str()
                        .expect("failed to convert to utf8")
                        .trim()
                }).unwrap_or_default();

                if link.starts_with("/wiki/") && !link.contains(":") {
                    Some(link.replace("/wiki/", ""))
                } else {
                    None
                }
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected_links() {
        let scraper = WikiScraper::new();
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
</p>"#;

        let expected_links = vec![
            "Topology​".into(),
            "Topological_space​".into(),
            "Real_line​".into(),
            "Lindel%C3%B6f_space​".into(),
            "Separable_space​".into(),
        ];
        let links = scraper.get_wiki_links(html).unwrap();
        // assert_eq!(links.len(), expected_links.len());
        assert_eq!(links, HashSet::from_iter(expected_links));
    }
}
