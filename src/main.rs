use rosalind::{Solvable,
               dna,
               fib,
               rna,
               revc,
               gc};

use std::fmt::Display;

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};


fn get_input<'a>(matches: &'a ArgMatches, subc_name: &'a str) -> &'a str {
    return matches.subcommand_matches(subc_name).unwrap().value_of("input").unwrap()
}

fn print_solved_problem<T: Solvable + Display >(matches: &ArgMatches, subc_name: &str, mut problem: T) {
    let input = matches.subcommand_matches(subc_name).unwrap().value_of("input").unwrap();
    problem.solve(input);
    println!("{}", problem);
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    match matches.subcommand_name() {
        Some("dna") => {
            let problem = dna::Problem::new();
            print_solved_problem(&matches, "dna", problem);
        },
        Some("fib") => {
            let problem = fib::Problem::new();
            print_solved_problem(&matches, "fib", problem);
        },
        Some("gc") => {
//            let problem = gc::Problem::new();
//            print_solved_problem(&matches, "gc", problem);
            println!("{}", gc::gc(gc::file_parse(get_input(&matches, "gc"))));
        },
        Some("rna") => {
//            let problem = rna::Problem::new();
//            print_solved_problem(&matches, "rna", problem);
            println!("{}", rna::rna(&rna::file_parse(get_input(&matches, "rna"))));
        },
        Some("revc") => {
            let problem = revc::Problem::new();
            print_solved_problem(&matches, "revc", problem);
//            println!("{}", revc::revc(&revc::file_parse(get_input(&matches, "revc"))));
        },
        _ => {},
    }

}
