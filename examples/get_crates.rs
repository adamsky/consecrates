use consecrates::{Client, Query};

fn main() {
    let client = Client::new("consecrates_example_client (github.com/adamsky/consecrates)");
    let crates = client
        .get_crates(Query {
            string: Some("example".to_string()),
            page: Some(1),
            per_page: Some(10),
            ..Default::default()
        })
        .expect("failed query");
    println!("{:?}", crates);
}
