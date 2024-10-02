#[macro_use]
extern crate log;

use clap::Parser;
use humantime::format_duration;
use itertools::Itertools;
use log::{info, warn};
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

    #[clap(short = 'e', long, required = true)]
    edges_input: path::PathBuf,

    #[clap(short = 'f', long, required = true)]
    edges_output: path::PathBuf,

    #[clap(short = 's', long, required = true)]
    subject_column_index: usize,

    #[clap(short = 'o', long, required = true)]
    object_column_index: usize,

    #[clap(short = 'c', long, default_value_t = 100)]
    number_of_lines: usize,
}

fn main() {
    env_logger::init();
    let start = Instant::now();

    let options = Options::parse();
    debug!("{:?}", options);

    write_truncated_edges_file(&options.edges_input, &options.edges_output, options.number_of_lines);

    let edges_output_file = std::fs::File::open(&options.edges_output.as_path()).expect("Could not open edges file");

    let reader = io::BufReader::new(edges_output_file);

    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(reader);
    let curies: Vec<String> = rdr.records().take(options.number_of_lines).skip(1).map(|result| {
        let record = result.expect("Could not get record");
        let subject = record.get(options.subject_column_index).unwrap();
        let object = record.get(options.object_column_index).unwrap();
        vec![subject.to_string(), object.to_string()]
    }).flatten().collect();

    debug!("curies.len(): {}", curies.len());

    let node_file_contents = std::fs::read_to_string(options.nodes_input).unwrap();

    let node_lines = node_file_contents.lines().skip(1).map(String::from).collect_vec();
    let get_line = |lines: &Vec<String>, x: &String| -> Option<String> { lines.par_iter().find_first(|line| line.starts_with(x)).cloned() };

    let mut nodes_writer = io::BufWriter::new(std::fs::File::create(options.nodes_output.as_path()).unwrap());

    for curie in curies {
        let line = get_line(&node_lines, &curie);
        match line {
            None => {
                warn!("{} not found", curie);
            }
            Some(l) => nodes_writer.write_all(format!("{}\n", l).as_bytes()).expect("Could not write node line"),
        };
    }
    info!("Duration: {}", format_duration(start.elapsed()).to_string());
}

fn write_truncated_edges_file(edges_input: &PathBuf, edges_output: &PathBuf, number_of_lines: usize) {
    let edges_file = std::fs::File::open(edges_input.as_path()).expect("Could not open edges file");
    let reader = io::BufReader::new(edges_file);
    let mut writer = io::BufWriter::new(std::fs::File::create(edges_output.as_path()).unwrap());
    reader.lines().take(number_of_lines).map(|line| line.unwrap()).for_each(|line| writer.write_all(format!("{}\n", line).as_bytes()).expect("Could not write edge line"));
}
