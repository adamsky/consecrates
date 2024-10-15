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

use std::convert::TryFrom;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use anyhow::{Error, Result};
use http_req::request::Request;
use http_req::uri::Uri;
use serde::de::DeserializeOwned;

use api::{
    Authors, Categories, Crate, Crates, Dependencies, Downloads, Keywords, Owners, Summary, Version,
};

/// Base url of the API.
const BASE_URL: &'static str = "https://crates.io/api/v1/";
/// Rate limit of one second is the smallest value tolerated by `crates.io`.
const RATE_LIMIT: Duration = Duration::from_secs(1);

/// API client abstraction.
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

    fn url_crates(&self, query: Query) -> Result<String> {
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

        Ok(url)
    }

    /// Gets a page of crates, using a set of query options.
    pub fn get_crates(&self, query: Query) -> Result<Crates> {
        let crates = self.get(&self.url_crates(query)?)?;
        Ok(crates)
    }

    /// Tries to get a page of crates, using a set of query options.
    pub fn try_get_crates(&self, query: Query) -> Result<Crates> {
        let crates = self.try_get(&self.url_crates(query)?)?;
        Ok(crates)
    }

    fn url_crate(&self, crate_id: &str) -> Result<String> {
        Ok(format!("{}crates/{}", self.base_url, crate_id))
    }

    /// Gets information about a particular crate.
    pub fn get_crate(&self, crate_id: &str) -> Result<Crate> {
        let crate_ = self.get(&self.url_crate(crate_id)?)?;
        Ok(crate_)
    }

    /// Tries to get information about a particular crate.
    pub fn try_get_crate(&self, crate_id: &str) -> Result<Crate> {
        let crate_ = self.try_get(&self.url_crate(crate_id)?)?;
        Ok(crate_)
    }

    fn url_crate_version(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        Ok(format!(
            "{}crates/{}/{}",
            self.base_url, crate_id, crate_version
        ))
    }

    /// Gets crate information for a particular version of the given crate.
    pub fn get_crate_version(&self, crate_id: &str, crate_version: &str) -> Result<Version> {
        let version = self.get(&self.url_crate_version(crate_id, crate_version)?)?;
        Ok(version)
    }

    /// Tries to get crate information for a particular version of the given
    /// crate.
    pub fn try_get_crate_version(&self, crate_id: &str, crate_version: &str) -> Result<Version> {
        let version = self.try_get(&self.url_crate_version(crate_id, crate_version)?)?;
        Ok(version)
    }

    fn url_crate_downloads(&self, crate_id: &str) -> Result<String> {
        Ok(format!("{}crates/{}/downloads", self.base_url, crate_id))
    }

    /// Gets information about the download stats for the given crate.
    pub fn get_crate_downloads(&self, crate_id: &str) -> Result<Downloads> {
        let downloads = self.get(&self.url_crate_downloads(crate_id)?)?;
        Ok(downloads)
    }

    /// Tries to get information about the download stats for the given crate.
    pub fn try_get_crate_downloads(&self, crate_id: &str) -> Result<Downloads> {
        let downloads = self.try_get(&self.url_crate_downloads(crate_id)?)?;
        Ok(downloads)
    }

    fn url_crate_dependencies(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        Ok(format!(
            "{}crates/{}/{}/dependencies",
            self.base_url, crate_id, crate_version
        ))
    }

    /// Gets a list of dependencies for a particular version of the given crate.
    pub fn get_crate_dependencies(
        &self,
        crate_id: &str,
        crate_version: &str,
    ) -> Result<Dependencies> {
        let dependencies = self.get(&self.url_crate_dependencies(crate_id, crate_version)?)?;
        Ok(dependencies)
    }

    /// Tries to get a list of dependencies for a particular version of the
    /// given crate.
    pub fn try_get_crate_dependencies(
        &self,
        crate_id: &str,
        crate_version: &str,
    ) -> Result<Dependencies> {
        let dependencies = self.get(&self.url_crate_dependencies(crate_id, crate_version)?)?;
        Ok(dependencies)
    }

    fn url_crate_owners(&self, crate_id: &str) -> Result<String> {
        Ok(format!("{}crates/{}/owners", self.base_url, crate_id))
    }

    /// Gets information about the owners of the given crate.
    pub fn get_crate_owners(&self, crate_id: &str) -> Result<Owners> {
        let owners = self.get(&self.url_crate_owners(crate_id)?)?;
        Ok(owners)
    }

    /// Tries to get information about the owners of the given crate.
    pub fn try_get_crate_owners(&self, crate_id: &str) -> Result<Owners> {
        let owners = self.try_get(&self.url_crate_owners(crate_id)?)?;
        Ok(owners)
    }

    fn url_crate_authors(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        Ok(format!(
            "{}crates/{}/{}/authors",
            self.base_url, crate_id, crate_version
        ))
    }

    /// Gets information about the authors for a particular version of the
    /// given crate.
    pub fn get_crate_authors(&self, crate_id: &str, crate_version: &str) -> Result<Authors> {
        let authors = self.get(&self.url_crate_authors(crate_id, crate_version)?)?;
        Ok(authors)
    }

    /// Tries to get information about the authors for a particular version of
    /// the given crate.
    pub fn try_get_crate_authors(&self, crate_id: &str, crate_version: &str) -> Result<Authors> {
        let authors = self.try_get(&self.url_crate_authors(crate_id, crate_version)?)?;
        Ok(authors)
    }

    fn url_crate_readme(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        Ok(format!(
            "{}crates/{}/{}/readme",
            self.base_url, crate_id, crate_version
        ))
    }

    /// Gets the readme for a particular version of the given crate.
    pub fn get_crate_readme(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        let readme = self.get(&self.url_crate_readme(crate_id, crate_version)?)?;
        Ok(readme)
    }

    /// Tries to get the readme for a particular version of the given crate.
    pub fn try_get_crate_readme(&self, crate_id: &str, crate_version: &str) -> Result<String> {
        let readme = self.try_get(&self.url_crate_readme(crate_id, crate_version)?)?;
        Ok(readme)
    }

    fn url_registry_summary(&self) -> Result<String> {
        Ok(format!("{}summary", self.base_url))
    }

    /// Gets registry-wide summary.
    pub fn get_registry_summary(&self) -> Result<Summary> {
        let summary = self.get(&self.url_registry_summary()?)?;
        Ok(summary)
    }

    /// Tries to get registry-wide summary.
    pub fn try_get_registry_summary(&self) -> Result<Summary> {
        let summary = self.try_get(&self.url_registry_summary()?)?;
        Ok(summary)
    }

    fn url_category(&self, query: Query) -> Result<String> {
        let mut cat_string = None;
        if let Some(s) = query.string {
            cat_string = Some(s);
        } else if let Some(cat) = query.category {
            cat_string = Some(cat.to_str().to_string());
        }

        if let Some(cats) = cat_string {
            let url = format!("{}categories/{}", self.base_url, &cats);
            Ok(url)
        } else {
            Err(Error::msg(
                "didn't provide either a string or category argument with query",
            ))
        }
    }

    /// Gets information about a category.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `string`
    /// or `category` fields.
    pub fn get_category(&self, query: Query) -> Result<api::Category> {
        let category = self.get(&self.url_category(query)?)?;
        Ok(category)
    }

    /// Tries to get information about a category.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `string`
    /// or `category` fields.
    pub fn try_get_category(&self, query: Query) -> Result<api::Category> {
        let category = self.try_get(&self.url_category(query)?)?;
        Ok(category)
    }

    fn url_categories(&self, query: Query) -> Result<String> {
        let mut url = self.base_url.clone();
        url.push_str("categories?");
        if let Some(page) = query.page {
            url.push_str(&format!("page={}", page));
        }
        if let Some(per_page) = query.per_page {
            url.push_str(&format!("&per_page={}", per_page));
        }
        Ok(url)
    }

    /// Gets a paged list of categories available with the registry.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `page`
    /// and `per_page` fields.
    pub fn get_categories(&self, query: Query) -> Result<Categories> {
        let categories = self.get(&self.url_categories(query)?)?;
        Ok(categories)
    }

    /// Tries to get a paged list of categories available with the registry.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `page`
    /// and `per_page` fields.
    pub fn try_get_categories(&self, query: Query) -> Result<Categories> {
        let categories = self.try_get(&self.url_categories(query)?)?;
        Ok(categories)
    }

    fn url_keyword(&self, query: Query) -> Result<String> {
        let mut key_string = None;
        if let Some(s) = query.string {
            key_string = Some(s);
        } else if let Some(key) = query.keyword {
            key_string = Some(key);
        }

        if let Some(keys) = key_string {
            let mut url = self.base_url.clone();
            url.push_str(&format!("keywords/{}", &keys));
            Ok(url)
        } else {
            Err(Error::msg(
                "didn't provide either a string or keyword argument with query",
            ))
        }
    }

    /// Gets information about a category.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `string`
    /// or `keyword` fields.
    pub fn get_keyword(&self, query: Query) -> Result<api::Keyword> {
        let keyword = self.get(&self.url_keyword(query)?)?;
        Ok(keyword)
    }

    /// Tries to get information about a category.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `string`
    /// or `keyword` fields.
    pub fn try_get_keyword(&self, query: Query) -> Result<api::Keyword> {
        let keyword = self.try_get(&self.url_keyword(query)?)?;
        Ok(keyword)
    }

    fn url_keywords(&self, query: Query) -> Result<String> {
        let mut url = self.base_url.clone();
        url.push_str("keywords?");
        if let Some(page) = query.page {
            url.push_str(&format!("page={}", page));
        }
        if let Some(per_page) = query.per_page {
            url.push_str(&format!("&per_page={}", per_page));
        }
        Ok(url)
    }

    /// Gets a paged list of keywords used by crates within the registry.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `page`
    /// and `per_page` fields.
    pub fn get_keywords(&self, query: Query) -> Result<Keywords> {
        let keywords = self.get(&self.url_keywords(query)?)?;
        Ok(keywords)
    }

    /// Tries to get a paged list of keywords used by crates within the
    /// registry.
    ///
    /// # Query details
    ///
    /// This function accepts a `Query` object but can only use it's `page`
    /// and `per_page` fields.
    pub fn try_get_keywords(&self, query: Query) -> Result<Keywords> {
        let keywords = self.try_get(&self.url_keywords(query)?)?;
        Ok(keywords)
    }

    fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        // block until it's been long enough since the last request
        loop {
            match self.try_get(url) {
                Err(error) => {
                    if std::io::ErrorKind::WouldBlock == error.kind() {
                        std::thread::sleep(Duration::from_millis(60));
                        continue;
                    } else {
                        return Err(Error::from(error));
                    }
                }
                Ok(response) => return Ok(response),
            }
        }
    }

    /// Tries to get data from the provided url.
    ///
    /// Only returns response body.
    ///
    /// # Semi-non-blocking
    ///
    /// Returns an error if client is waiting for rate limiter to allow
    /// processing next request. Processing http request itself will block.
    fn try_get<T: DeserializeOwned>(&self, url: &str) -> std::io::Result<T> {
        let mut lr = self.last_request.lock().unwrap();
        if lr.elapsed() >= RATE_LIMIT {
            *lr = Instant::now();
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                Error::msg("Would block"),
            ));
        }
        let mut buffer = Vec::new();
        let uri =
            Uri::try_from(url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let _ = Request::new(&uri)
            .header("User-Agent", &self.user_agent)
            .send(&mut buffer)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let deser: T = serde_json::from_slice(&buffer)?;
        Ok(deser)
    }
}
