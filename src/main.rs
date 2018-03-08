#[macro_use]
extern crate quicli;
use quicli::prelude::*;

#[derive(Debug, StructOpt)]
struct Cli {
  #[structopt(long = "verbosity", short = "v", parse(from_occurrences))]
  verbosity: u8,
}

main!(|args: Cli, log_level: verbosity| {
  let world = "world";
  println!("Hello {}", world);
});
