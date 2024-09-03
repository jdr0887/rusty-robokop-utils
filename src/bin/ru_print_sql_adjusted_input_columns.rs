use itertools::Itertools;
use std::path::PathBuf;

fn main() {
    print_data(&std::path::PathBuf::from("/home/jdr0887/Downloads/RobokopKG/robokop_kg_edges.csv"));
    print_data(&std::path::PathBuf::from("/home/jdr0887/Downloads/RobokopKG/robokop_kg_nodes.csv"));
}

fn print_data(file: &PathBuf) {
    let file_contents = std::fs::read_to_string(file).unwrap();
    let header = file_contents.lines().next();

    let orig_header = header
        .unwrap()
        .split("\t")
        .map(|orig| {
            let orig = orig
                .replace("[", "_")
                .replace("]", "_")
                .replace(".", "_")
                .replace(",", "_")
                .replace("/", "_")
                .replace(";", "_")
                .replace("*", "_")
                .replace("{", "_")
                .replace("}", "_");
            let new = orig.split(":").collect_vec();
            new.get(0).unwrap().to_string()
        })
        .collect_vec();

    println!("{}", orig_header.join("\t"));
}
