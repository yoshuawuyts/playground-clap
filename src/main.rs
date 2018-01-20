extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use std::fmt;

#[derive(StructOpt, Debug)]
struct Cli {}

impl fmt::Display for Cli {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", "hi", "hello")
  }
}

fn main() {
  let foo = Cli::from_args();
  println!("{}", foo);
}
