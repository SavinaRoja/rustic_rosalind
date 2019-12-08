use std::fs;

#[macro_use]
extern crate clap;
use clap::App;

fn dna(filename :&str) {
    let file_contents = fs::read_to_string(filename).unwrap();

    let mut a_count : u32 = 0;
    let mut c_count : u32 = 0;
    let mut g_count : u32 = 0;
    let mut t_count : u32 = 0;

    for char in file_contents.chars() {
        match char {
            'A' => {a_count += 1},
            'C' => {c_count += 1},
            'G' => {g_count += 1},
            'T' => {t_count += 1},
            _ => {},  // To make match exhaustive
        }
    }

    println!("{} {} {} {}", a_count, c_count, g_count, t_count);
}

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
            dna(input_file);
        },
        Some("rna") => {
            let input_file = matches.subcommand_matches("rna").unwrap().value_of("input").unwrap();
            rna(input_file);
        },
        _ => {},
    }

}
