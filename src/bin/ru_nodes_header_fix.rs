#[macro_use]
extern crate log;

use clap::Parser;
use humantime::format_duration;
use itertools::Itertools;
use rayon::prelude::*;
use std::io;
use std::io::prelude::*;
use std::path;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, PartialEq, Debug)]
#[clap(author, version, about, long_about = None)]
struct Options {
    #[clap(short = 'n', long, required = true)]
    nodes_input: path::PathBuf,

    #[clap(short = 'm', long, required = true)]
    nodes_output: path::PathBuf,
}

fn main() {
    let start = Instant::now();
    env_logger::init();

    let options = Options::parse();
    debug!("{:?}", options);

    let nodes_input_file = std::fs::File::open(options.nodes_input.as_path()).expect("Could not open edges file");
    let reader = io::BufReader::new(nodes_input_file);

    let header = reader.lines().take(1).next().expect("Could not get header").unwrap();
    header.split("\t").filter(|col| col.starts_with("MONDO_SUPERCLASS")).map(|col| col.clone()).for_each(|col| println!("{}", col));
    println!("------------");
    header.split("\t").filter(|col| col.starts_with("CHEBI_ROLE")).map(|col| col.clone()).for_each(|col| println!("{}", col));
    println!("------------");
    header.split("\t").filter(|col| !col.starts_with("CHEBI_ROLE")).filter(|col| !col.starts_with("MONDO_SUPERCLASS")).map(|col| col.clone()).for_each(|col| println!("{}", col));

    info!("Duration: {}", format_duration(start.elapsed()).to_string());
}
