use humantime::format_duration;
use itertools::Itertools;
use log::{info, warn};
use rayon::prelude::*;
use std::io::prelude::*;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    env_logger::init();
    let start = Instant::now();

    let input = &std::path::PathBuf::from("/media/jdr0887/backup/home/jdr0887/matrix/robokop_kg_nodes_test.csv");

    let usable_columns = get_usable_columns(input);
    println!("usable_columns: {:?}", usable_columns);

    let input_file = std::fs::File::open(input.as_path()).expect("Could not open edges file");
    let reader = std::io::BufReader::new(input_file);
    reader.lines().for_each(|line| {
        println!("----------");
        let line = line.unwrap();
        let line_split = line.split("\t").collect_vec();
        usable_columns.iter().cloned().for_each(|(idx, col)| {
            let value = line_split.get(idx).unwrap();
            if col.eq("category") || col.eq("equivalent_identifiers") {
                println!("{}: {:?}", col, value.split(char::from_u32(0x0000001F).unwrap()).collect_vec());
            } else {
                println!("{}: {}", col, value);
            }
        });
    });
    info!("Duration: {}", format_duration(start.elapsed()).to_string());
}

fn get_usable_columns(input: &PathBuf) -> Vec<(usize, String)> {
    let input_file = std::fs::File::open(input.as_path()).expect("Could not open edges file");
    let reader = std::io::BufReader::new(input_file);

    let header = reader.lines().take(1).next().expect("Could not get header").unwrap();
    // println!("header: {}", header);
    let mut usable_columns = vec![];
    header.split("\t").enumerate().for_each(|(idx, col)| {
        if !col.starts_with("CHEBI_ROLE") && !col.starts_with("MONDO_SUPERCLASS") {
            let col_name_split = col.split(":").collect_vec();
            let col_name = col_name_split.get(0).unwrap();
            usable_columns.push((idx, col_name.to_string()));
        }
    });
    usable_columns
}
