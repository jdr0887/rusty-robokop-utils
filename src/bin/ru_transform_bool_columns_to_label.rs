#[macro_use]
extern crate log;

use rayon::prelude::*;
use clap::Parser;
use humantime::format_duration;
use itertools::Itertools;
use std::error::Error;
use std::fs;
use std::io::{BufRead, Write};
use std::path;
use std::time::Instant;

#[derive(Parser, PartialEq, Debug)]
#[clap(author, version, about, long_about = None)]
struct Options {
    #[clap(short, long, required = true)]
    input: path::PathBuf,

    #[clap(short, long, required = true)]
    output: path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    env_logger::init();

    let options = Options::parse();
    debug!("{:?}", options);

    let input = options.input;
    let output = options.output;

    let input_file = fs::File::open(input.as_path()).expect("Could not open input file");
    let reader = std::io::BufReader::new(&input_file);

    let header = reader.lines().take(1).next().expect("Could not get header").unwrap();
    // println!("header: {}", header);

    let header_columns = header.split("\t").collect_vec();

    let keep_columns = header_columns.iter()
        .enumerate()
        .filter(|(_idx, col)| !col.starts_with("CHEBI_ROLE") && !col.starts_with("MONDO_SUPERCLASS"))
        .map(|(idx, col)| {
            let col_name_split = col.split(":").collect_vec();
            let col_name = col_name_split.get(0).unwrap();
            (idx, col_name.to_string())
        })
        .collect_vec();

    let mut new_header = keep_columns.iter().map(|(_idx, col)| col.clone()).collect_vec().join("\t");
    new_header.push_str("\tCHEBI_ROLE");
    new_header.push_str("\tMONDO_SUPERCLASS");

    let chebi_role_colums: Vec<(usize, String)> = header_columns.iter()
        .enumerate()
        .filter(|(_idx, col)| col.starts_with("CHEBI_ROLE"))
        .map(|(idx, col)| {
            let mut ret = None;
            if let Some((prefix, _suffix)) = col.split_once(':') {
                ret = Some((idx, prefix.replace("CHEBI_ROLE_", "")));
            }
            ret
        }).filter_map(std::convert::identity)
        .collect_vec();

    let mondo_superclass_colums: Vec<(usize, String)> = header_columns.iter()
        .enumerate()
        .filter(|(_idx, col)| col.starts_with("MONDO_SUPERCLASS"))
        .map(|(idx, col)| {
            let mut ret = None;
            if let Some((prefix, _suffix)) = col.split_once(':') {
                ret = Some((idx, prefix.replace("MONDO_SUPERCLASS_", "")));
            }
            ret
        }).filter_map(std::convert::identity)
        .collect_vec();

    let mut writer = std::io::BufWriter::new(fs::File::create(output.as_path()).unwrap());
    writer.write_all(format!("{}\n", new_header).as_bytes()).expect("Could not write line");

    let separator = char::from_u32(0x0000001F).unwrap();

    let reader = std::io::BufReader::new(&input_file);
    reader.lines().skip(1).for_each(|line| {
        let line = line.unwrap();
        let line_split = line.split("\t").collect_vec();

        let mut new_line = String::new();
        keep_columns.iter().cloned().for_each(|(idx, _col)| {
            let value = line_split.get(idx).unwrap();
            new_line.push_str(format!("{}\t", value).as_str());
        });

        let chebi_role_labels: Vec<&str> = chebi_role_colums
            .par_iter()
            .filter_map(|(idx, col)| {
                let mut ret = None;
                if let Some((prefix, _suffix)) = col.split_once(':') {
                    let value = line_split.get(idx.clone()).unwrap();
                    if "true".eq(*value) {
                        ret = Some(prefix)
                    }
                }
                ret
            })
            .collect();
        new_line.push_str(format!("{}\t", chebi_role_labels.into_iter().join(format!("{}", separator).as_str())).as_str());

        let mondo_superclass_labels: Vec<&str> = mondo_superclass_colums
            .par_iter()
            .filter_map(|(idx, col)| {
                let mut ret = None;
                if let Some((prefix, _suffix)) = col.split_once(':') {
                    let value = line_split.get(idx.clone()).unwrap();
                    if "true".eq(*value) {
                        ret = Some(prefix)
                    }
                }
                ret
            })
            .collect();
        new_line.push_str(mondo_superclass_labels.into_iter().join(format!("{}", separator).as_str()).as_str());

        writer.write_all(format!("{}\n", new_line).as_bytes()).expect("Could not write line");
    });

    info!("Duration: {}", format_duration(start.elapsed()).to_string());
    Ok(())
}
