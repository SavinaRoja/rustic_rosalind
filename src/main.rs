use std::fs;

use rosalind::{dna, rna, revc};
use rosalind::fib;

#[macro_use]
extern crate clap;
use clap::App;


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

//    let input_file = matches.value_of("input").unwrap();

    match matches.subcommand_name() {
        Some("dna") => {
            let input_file = matches.subcommand_matches("dna").unwrap().value_of("input").unwrap();
            let contents = fs::read_to_string(input_file).unwrap();
            println!("{}", dna(&contents));
        },
        Some("fib") => {
            let input_file = matches.subcommand_matches("fib").unwrap().value_of("input").unwrap();
            println!("{}", fib(fib::file_parse(input_file)));
        },
        Some("rna") => {
            let input_file = matches.subcommand_matches("rna").unwrap().value_of("input").unwrap();
            let contents = fs::read_to_string(input_file).unwrap();
            println!("{}", rna(&contents));
        },
        Some("revc") => {
            let input_file = matches.subcommand_matches("revc").unwrap().value_of("input").unwrap();
            let contents = fs::read_to_string(input_file).unwrap();
            println!("{}", revc(&contents));
        },
        _ => {},
    }

}
