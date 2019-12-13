use rosalind::{Solvable,
               dna,
               fib,
               gc,
               hamm,
               rna,
               revc};

use std::fmt::Display;

#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

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
            let problem = gc::Problem::new();
            print_solved_problem(&matches, "gc", problem);
        },
        Some("hamm") => {
            let problem = hamm::Problem::new();
            print_solved_problem(&matches, "hamm", problem);
        },
        Some("rna") => {
            let problem = rna::Problem::new();
            print_solved_problem(&matches, "rna", problem);
        },
        Some("revc") => {
            let problem = revc::Problem::new();
            print_solved_problem(&matches, "revc", problem);
        },
        _ => {},
    }

}
