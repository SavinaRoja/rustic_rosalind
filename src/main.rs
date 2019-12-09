use rosalind::{dna, fib, rna, revc, gc};

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};


fn get_input<'a>(matches: &'a ArgMatches, subc_name: &'a str) -> &'a str {
    return matches.subcommand_matches(subc_name).unwrap().value_of("input").unwrap()
}


fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand_name() {
        Some("dna") => {
            println!("{}", dna::dna(&dna::file_parse(get_input(&matches, "dna"))));
        },
        Some("fib") => {
            println!("{}", fib::fib(fib::file_parse(get_input(&matches, "fib"))));
        },
        Some("gc") => {
            println!("{}", gc::gc(gc::file_parse(get_input(&matches, "gc"))));
        },
        Some("rna") => {
            println!("{}", rna::rna(&rna::file_parse(get_input(&matches, "rna"))));
        },
        Some("revc") => {
            println!("{}", revc::revc(&revc::file_parse(get_input(&matches, "revc"))));
        },
        _ => {},
    }

}
