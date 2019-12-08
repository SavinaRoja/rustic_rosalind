use std::fs;

use rosalind::dna;

#[macro_use]
extern crate clap;
use clap::App;

fn rna(filename: &str) {
    let file_contents = fs::read_to_string(filename).unwrap();
    let translated: String = file_contents.chars().map(|x| {
        match x {
            'T' => {'U'},
            _ => {x}
        }
    }).collect();
    println!("{}", translated);
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

//    let input_file = matches.value_of("input").unwrap();

    match matches.subcommand_name() {
        Some("dna") => {
            let input_file = matches.subcommand_matches("dna").unwrap().value_of("input").unwrap();
            let contents = fs::read_to_string(input_file).unwrap();
            let vals = dna(contents);
            println!("{}", vals);
        },
        Some("rna") => {
            let input_file = matches.subcommand_matches("rna").unwrap().value_of("input").unwrap();
            rna(input_file);
        },
        _ => {},
    }

}
