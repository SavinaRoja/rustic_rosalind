use std::fmt;
use std::fs;
use super::Solvable;
use std::fmt::{Formatter, Error};

pub type Problem = RNAResult;

#[derive(Debug, PartialEq)]
pub struct RNAResult {
    val: String,
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.val)
    }
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { val: String::new() }
    }
}

impl Solvable for Problem {
    type Parameters = String;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        return fs::read_to_string(input_filename).unwrap();
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        self.val = params.chars().map(|x| {
            match x {
                'T' => {'U'},
                _ => {x}
            }
        }).collect();
    }
}


pub fn rna(input: &String) -> String {
    let translated: String = input.chars().map(|x| {
        match x {
            'T' => {'U'},
            _ => {x}
        }
    }).collect();
    return translated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rna_empty() {
        let mut problem = Problem::new();
        problem.get_solution(String::new());
        assert_eq!(problem, Problem { val: String::new() });
    }

    #[test]
    fn rna_no_action() {
        let mut problem = Problem::new();
        let input = String::from("ACGACG");
        let copy = input.clone();
        problem.get_solution(input);
        assert_eq!(problem, Problem { val: copy });
    }

    #[test]
    fn rna_minimal_action() {
        let mut problem = Problem::new();
        problem.get_solution(String::from("T"));
        assert_eq!(problem, Problem { val: String::from("U") });
    }

    #[test]
    fn rna_simple() {
        let input = String::from("ACGTACGT");
        let result = String::from("ACGUACGU");
        let mut problem = Problem::new();
        problem.get_solution(input);
        assert_eq!(problem, Problem { val: result });
    }

    #[test]
    fn rna_with_a_full_dataset() {
        let input = String::from("\
AAGCCCCCTCATCGATAGCAGCCGTACCCCTGGCGGCCGACGAATGTGGGCTAACTTTGACTACGCGTCGTCACCGGCT\
GAAAAGTGCACATTAGGTCAGTTGGTTCAAGCAGGCCGAGACACTCGGCTATCAGACAAAGTTACTCCGTCGCACCGCC\
GCGGGTGATAATAGCCAAGAAAGTTCAAGTTGCTCGAAGTCTCTCCAAGCCGTAGTACAGTTTGCTTCCCGCCTGCAGG\
CCTGCTGTATGCTTACAAGATGGGCCACGTGTAAAGGATGGATTTCCAAAAAGAAACACCCTAGGTGTACATTAAGATC\
TTTACGCCGCGAGGGCATCAACGATTATAGGAAATTCAGAATCGGCAGTCATTCTGACTCCGTCATCGGCCCTCTTGAT\
CTCTATTCTGGACAGCTTCTACGTACTTAAAAGGTCTCGCACGCCACTACGTAATCTCGGCTCAGGAAGGCGCATGTGG\
GAGAGCCTATCGCTAGAGGCGCATGTCTTCGCTAGCACTCAAATTAGACAGGATTATTTTTAGTTGCGCGTGACTCTGC\
TTGGCCTCACACAAGGTTCCCACCTCATGGTTAAAGTATTATGCCCTCAGCCAATGATGCCCCGAAAGATTCACCTCCA\
TGATCGGTAACTTGCACCCTTCAGCCCCCTTATAAACAGCAGCAATACTAGGATATACGTTCTTCGTATTCAATCATCC\
TATCATCTAATCCATTAGGCTTTCCGCTCTGTTTACCATCATACCCACAGAGGCCAGCAGCCCTTCTGCTTACGAAGCC\
CTGTAACCAAAGTAGTGGGGTTGCCGTTTTTCATCCGAGATGAACGCCAGACCCCGCGCACTTCGCCCGGTTGCGTTCC\
AATTACAGTCATCTTGTGATAAGTCCGGGGTACAGGGCACCGGTGCTCCGAGG");
        let result = String::from("\
AAGCCCCCUCAUCGAUAGCAGCCGUACCCCUGGCGGCCGACGAAUGUGGGCUAACUUUGACUACGCGUCGUCACCGGCU\
GAAAAGUGCACAUUAGGUCAGUUGGUUCAAGCAGGCCGAGACACUCGGCUAUCAGACAAAGUUACUCCGUCGCACCGCC\
GCGGGUGAUAAUAGCCAAGAAAGUUCAAGUUGCUCGAAGUCUCUCCAAGCCGUAGUACAGUUUGCUUCCCGCCUGCAGG\
CCUGCUGUAUGCUUACAAGAUGGGCCACGUGUAAAGGAUGGAUUUCCAAAAAGAAACACCCUAGGUGUACAUUAAGAUC\
UUUACGCCGCGAGGGCAUCAACGAUUAUAGGAAAUUCAGAAUCGGCAGUCAUUCUGACUCCGUCAUCGGCCCUCUUGAU\
CUCUAUUCUGGACAGCUUCUACGUACUUAAAAGGUCUCGCACGCCACUACGUAAUCUCGGCUCAGGAAGGCGCAUGUGG\
GAGAGCCUAUCGCUAGAGGCGCAUGUCUUCGCUAGCACUCAAAUUAGACAGGAUUAUUUUUAGUUGCGCGUGACUCUGC\
UUGGCCUCACACAAGGUUCCCACCUCAUGGUUAAAGUAUUAUGCCCUCAGCCAAUGAUGCCCCGAAAGAUUCACCUCCA\
UGAUCGGUAACUUGCACCCUUCAGCCCCCUUAUAAACAGCAGCAAUACUAGGAUAUACGUUCUUCGUAUUCAAUCAUCC\
UAUCAUCUAAUCCAUUAGGCUUUCCGCUCUGUUUACCAUCAUACCCACAGAGGCCAGCAGCCCUUCUGCUUACGAAGCC\
CUGUAACCAAAGUAGUGGGGUUGCCGUUUUUCAUCCGAGAUGAACGCCAGACCCCGCGCACUUCGCCCGGUUGCGUUCC\
AAUUACAGUCAUCUUGUGAUAAGUCCGGGGUACAGGGCACCGGUGCUCCGAGG");
        let mut problem = Problem::new();
        problem.get_solution(input);
        assert_eq!(problem, Problem { val: result });
    }
}