use anyhow::{Context, Result};

pub struct Html(pub(crate) String);
impl Html {
    pub fn new(html: String) -> Self {
        Self(html)
    }
}

pub trait PageFetcher {
  fn fetch(&self, page_name: &str) -> Result<Html>;
}

pub struct WikiPageFetcher {
  base_url: String,
}

impl WikiPageFetcher {
  fn new() -> Self {
      Self {
          base_url: "https://en.wikipedia.org".to_string(),
      }
  }
}

impl Default for WikiPageFetcher {
  fn default() -> Self {
      Self::new()
  }
}

impl PageFetcher for WikiPageFetcher {
  fn fetch(&self, page_name: &str) -> Result<Html> {
      let url = format!("{}/wiki/{}", self.base_url, page_name);
      let response = reqwest::blocking::get(&url).context(format!("failed to fetch page {}", url))?;
      let html = response.text().context("Failed to parse response")?;
      Ok(Html::new(html))
  }
}
