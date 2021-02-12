use consecrates::{Client, Query};

fn main() {
    let client = Client::new("consecrates_example_client (github.com/adamsky/consecrates)");
    let _ = client
        .crates(Query {
            string: Some("example".to_string()),
            page: Some(1),
            per_page: Some(10),
            ..Default::default()
        })
        .expect("failed query");
    // println!("{}", response_string);
}
