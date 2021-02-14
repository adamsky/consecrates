# consecrates

Tiny but virtuous [crates.io](https://crates.io) API client.

The main aim of this library is to provide an easy way to query crates
information without bringing in too many dependencies.

It's loosely modeled after the
[crates_io_api](https://crates.io/crates/crates_io_api) crate. Main differences
include:
- about 70% cut in the number of dependencies
- no async 
- no multi-request client methods like `full_crate` or
  `all_crates`
- ability to use `category` and `keyword` specifiers for querying crates
- ability to convert simple string composite queries such as
  `api category=web keyword=crates sort=update` into valid query objects


## Using

Paste the following into your project's `Cargo.toml` file:

```toml
consecrates = "0.1.0"
```

Create a new client and issue a query: 

```rust,no_run
let client = Client::new("my_app (github.com/me/me_app)");
let crates = client
    .get_crates(Query {
        string: Some("net".to_string()),
        category: Some(Category::GameDevelopment),
        sort: Some(Sorting::RecentUpdates),
        ..Default::default()
    })
    .expect("failed getting crates");
println!("{:?}", crates);
```


## Crawler policy

Please consult the
[official crawler policy](https://crates.io/policies#crawlers) before using
this library. Rate limiting is fixed at the lowest tolerated value. When
creating a client you will need to input a proper user-agent string.


