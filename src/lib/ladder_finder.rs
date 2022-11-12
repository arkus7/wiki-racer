use std::collections::VecDeque;

use anyhow::Result;
use priority_queue::PriorityQueue;

use crate::{WikiScraper, PageFetcher};

pub struct WikiLadderFinder<F: PageFetcher> {
    scraper: WikiScraper<F>,
}

impl<F: PageFetcher> WikiLadderFinder<F> {
    pub fn with_fetcher(fetcher: F) -> Self {
        Self {
          scraper: WikiScraper::with_fetcher(fetcher),
        }
    }

    pub fn find_ladder(&self, start_page: &str, end_page: &str) -> Result<Vec<String>> {
      let mut visited_pages = vec![];
      let mut queue = PriorityQueue::new();

      let end_page_links = self.scraper.get_wiki_links(end_page)?;

      queue.push(vec![start_page.to_string()], 0);

      while !queue.is_empty() {
        let (mut ladder, _prio) = queue.pop().unwrap();
        // dbg!(&ladder, prio);
        let last_page = ladder.last().expect("ladder has at least one element");
        let links = self.scraper.get_wiki_links(last_page)?;
        visited_pages.push(last_page.to_owned());
        let common_links_count = end_page_links.intersection(&links).count();

        // println!("Visiting page {} (common links: {})", last_page, common_links_count);
        dbg!(&ladder);

        if links.contains(end_page) {
          ladder.push(end_page.to_string());
          return Ok(ladder);
        }

        for link in links {
          if !visited_pages.contains(&link) {
            let mut ladder = ladder.clone();
            ladder.push(link.clone());
            queue.push(ladder, common_links_count);
          }
        }
      }

      Err(anyhow::anyhow!("No ladder found"))
    }
}

impl<F: PageFetcher + Default> Default for WikiLadderFinder<F> {
    fn default() -> Self {
        Self::with_fetcher(F::default())
    }
  }
