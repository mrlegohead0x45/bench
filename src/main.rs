mod args;
use std::collections::HashSet;
use std::time;

use args::Args;

use clap::Parser;

fn main() {
    let start = time::Instant::now();
    let args = Args::parse();
    let mut values: HashSet<time::Duration>;

    let elapsed = start.elapsed();
    println!("{:?}", args);
}
