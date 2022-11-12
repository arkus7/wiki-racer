use wiki_racer::WikiScraper;
use wiki_racer::WikiPageFetcher;

fn main() {
    let scraper = WikiScraper::with_fetcher(WikiPageFetcher::default());
    let links = scraper.get_wiki_links("Rust").unwrap();

    dbg!(links);
}
