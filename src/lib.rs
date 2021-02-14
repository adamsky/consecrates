//! Tiny but virtuous [crates.io](https://crates.io) API client.
//!
//! The main aim of this library is to provide an easy way to query crates
//! information without bringing in too many dependencies.
//!
//! It's loosely modeled after the
//! [crates_io_api](https://crates.io/crates/crates_io_api) crate. Main differences
//! include:
//! - about 70% cut in the number of dependencies
//! - no async
//! - no multi-request client methods like `full_crate` or
//!   `all_crates`
//! - ability to use `category` and `keyword` specifiers for querying crates
//! - ability to convert simple string composite queries such as
//!   `api category=web keyword=crates sort=update` into valid query objects
//!
//!
//! # Using
//!
//! Paste the following into your project's `Cargo.toml` file:
//!
//! ```toml
//! consecrates = "0.1.0"
//! ```
//!
//! Create a new client and issue a query:
//!
//! ```rust,no_run
//! # use consecrates::{Client, Query, Category, Sorting};
//! let client = Client::new("my_app (github.com/me/me_app)");
//! let crates = client
//!     .get_crates(Query {
//!         string: Some("net".to_string()),
//!         category: Some(Category::GameDevelopment),
//!         sort: Some(Sorting::RecentUpdates),
//!         ..Default::default()
//!     })
//!     .expect("failed getting crates");
//! println!("{:?}", crates);
//! ```
//!
//!
//! # Crawler policy
//!
//! Please consult the
//! [official crawler policy](https://crates.io/policies#crawlers) before using
//! this library. Rate limiting is fixed at the lowest tolerated value. When
//! creating a client you will need to input a proper user-agent string.

#[macro_use]
extern crate serde;

pub mod api;
mod query;

pub use query::{Category, Query, Sorting};

use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::Result;
use http_req::request::Request;
use http_req::uri::Uri;

use api::{Crates, Version};

/// Base url of the API.
const BASE_URL: &'static str = "https://crates.io/api/v1/";
/// Rate limit of one second is the smallest value tolerated by `crates.io`.
const RATE_LIMIT: Duration = Duration::from_secs(1);

/// Basic client.
pub struct Client {
    /// Base url used by the client
    base_url: String,
    /// User-Agent header used by the client
    user_agent: String,
    /// Time of the last request performed by the client
    last_request: Mutex<Instant>,
}

impl Client {
    /// Creates a new client with the given user agent string.
    ///
    /// # User-agent requirement
    ///
    /// `crates.io` requires the requests to include a user-agent header.
    /// Here are a few examples of proper user-agent strings you can use:
    /// ```text
    /// my_crawler (my_crawler.com/info)
    /// my_crawler (help@my_crawler.com)
    /// my_crawler (github.com/me/my_crawler)
    /// ```
    pub fn new(user_agent: &str) -> Self {
        Self::new_with_base_url(BASE_URL, user_agent)
    }

    /// Creates a new client with the given base url and user agent string.
    pub fn new_with_base_url(base_url: &str, user_agent: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            user_agent: user_agent.to_string(),
            last_request: Mutex::new(Instant::now() - RATE_LIMIT),
        }
    }

    /// Gets a page of crates, using a set of query options.
    pub fn get_crates(&self, query: Query) -> Result<Crates> {
        // construct the target url
        let mut url = self.base_url.clone();
        url.push_str("crates?");
        if let Some(page) = query.page {
            url.push_str(&format!("page={}", page));
        }
        if let Some(per_page) = query.per_page {
            url.push_str(&format!("&per_page={}", per_page));
        }
        if let Some(sort) = query.sort {
            url.push_str(&format!("&sort={}", sort.to_str()));
        }
        if let Some(query_string) = query.string {
            url.push_str(&format!("&q={}", query_string));
        }
        if let Some(cat) = query.category {
            url.push_str(&format!("&category={}", cat.to_str()));
        }
        if let Some(keyword) = query.keyword {
            url.push_str(&format!("&keyword={}", keyword))
        }

        // get the data
        let data = self.get(&url)?;
        let resp: Crates = serde_json::from_slice(&data)?;
        Ok(resp)
    }

    /// Gets crate information for a particular version of the given crate.
    pub fn get_crate_version(&self, crate_id: &str, crate_version: &str) -> Result<Version> {
        let mut url = self.base_url.clone();
        url.push_str(&format!("crates/{}/{}", crate_id, crate_version));
        let data = self.get(&url)?;
        let version = serde_json::from_slice(&data)?;
        Ok(version)
    }

    /// Gets the readme for the given crate.
    pub fn get_readme(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        let mut url = self.base_url.clone();
        url.push_str(&format!("crates/{}/{}/readme", crate_id, crate_version));
        let data = self.get(&url)?;
        let readme = String::from_utf8(data)?;
        Ok(readme)
    }

    /// Gets data from the provided url.
    ///
    /// Only returns response body.
    pub fn get(&self, url: &str) -> Result<Vec<u8>> {
        // honor the rate limit
        loop {
            let mut lr = self.last_request.lock().unwrap();
            if lr.elapsed() >= RATE_LIMIT {
                *lr = Instant::now();
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        let mut buffer = Vec::new();
        let uri: Uri = url.parse()?;
        let _ = Request::new(&uri)
            .header("User-Agent", &self.user_agent)
            .send(&mut buffer)?;
        Ok(buffer)
    }
}
