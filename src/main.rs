use std::fs;

use rosalind::{dna, fib, rna, revc};

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};


fn get_input<'a>(matches: &'a ArgMatches, subc_name: &'a str) -> &'a str {
    return matches.subcommand_matches(subc_name).unwrap().value_of("input").unwrap()
}


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

//    let input_file = matches.value_of("input").unwrap();

    match matches.subcommand_name() {
        Some("dna") => {
            println!("{}", dna::dna(&dna::file_parse(get_input(&matches, "dna"))));
        },
        Some("fib") => {
            println!("{}", fib::fib(fib::file_parse(get_input(&matches, "fib"))));
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
