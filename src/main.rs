extern crate alfred;
#[macro_use]
extern crate quicli;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use quicli::prelude::*;
use std::io;
use alfred::ItemBuilder;
use models::Gem;

mod models;

fn search_gems(query: String) -> Result<Vec<Gem>> {
  let url = &format!("https://rubygems.org/api/v1/search?query={}", query);
  let response: Vec<Gem> = reqwest::get(url)?.json()?;

  Ok(response)
}

fn placeholder_item() -> io::Result<()> {
  let item = ItemBuilder::new("Search for your favorite Gems")
    .arg("https://rubygems.org")
    .into_item();

  alfred::json::write_items(io::stdout(), &[item])
}

#[derive(Debug, StructOpt)]
struct Cli {
  query: String,
}

main!(|args: Cli| {
  let query = args.query;

  if query.is_empty() {
    placeholder_item()?;
  } else {
    let gems = search_gems(query)?;
    println!("{:?}", gems)
  }
});
