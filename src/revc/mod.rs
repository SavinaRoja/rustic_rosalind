use std::fs;
use std::fmt;
use super::Solvable;
use std::fmt::{Formatter, Error};

pub type Problem = REVCSolution;

#[derive(Debug, PartialEq)]
pub struct REVCSolution {
    val : String,
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { val: String::new() }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.val)
    }
}

impl Solvable for Problem {
    type Parameters = String;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        return fs::read_to_string(input_filename).unwrap();
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        self.val = params.chars()
        .rev()
        .filter(|x| {
            match x {
                'A' => true,
                'C' => true,
                'G' => true,
                'T' => true,
                _ => false,
            }
        })
        .map(
        |x| {
            match x {
                'A' => 'T',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => x,
            }
        }
    ).collect();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revc_empty() {
        let mut problem = Problem::new();
        let input = String::new();
        problem.get_solution(input);
        assert_eq!(problem, Problem::new());
    }

    #[test]
    fn revc_self_reverse_complement() {
        let mut problem = Problem::new();
        let input = String::from("AACCGGTT");
        let copy = input.clone();
        problem.get_solution(input);
        assert_eq!(problem, Problem { val: copy })
    }

    #[test]
    fn revc_simple_complement() {
        let mut problem = Problem::new();
        let input = String::from("AA");
        problem.get_solution(input);
        assert_eq!(problem, Problem { val: String::from("TT") });
    }

    #[test]
    fn revc_with_a_full_dataset() {
        let input = String::from("\
TTGAGCCTCGCGCAGTGTCTTTATCCGAGGGGGTCTTAGGGAATGGCTTGGCGTCTAACTCCAAATCGCCAGTTGGTCAA\
ATCGCTATAATAGTTGCCTCCACGAAGGTCGCAAGGACTGGGGGTAGATGCACGCTGACATGGAGCACAGTCGTCCACCA\
TAGATGCCATTACGCTAGCTCCGTAGATTAATCAGTCGGATGTTCACCAATACCCCATCATCTAGCGCTTTGGCGCGAGA\
CCCTACAAACTTTGCTTAGTCTTTTAGAGGATGATATTCAAACATTAAATATGTTCGATACATATACACTCTACAGTGTC\
CGTTCCGTGTGATGGGGTCCCTATTTTAATTTAAATATCTTCTGGCCAGGCGCAGCTCTACAGCCAGAGGTGAACATTTG\
GCAGAAATTCTAGTGAAGTGATGTCCCCCTTGAAGATAACTAAAGATATTATAAGCTTACGAGGAAGATGGTGAATTGGC\
GCTATATACCACTGGAACTGTCGGGACAATCCTAGGCCTCTGCGGAGACAACGATAATACATCCAATTAAATGCTCAGGG\
CTATCCTGGGCAGGCCTATGGGGCGGTGCCGATGTGAGACACAGCCTCGCGCTGATATGCAGGTTCAATGTTTTTAAAGA\
ATCCGACTACGCTCAAGGTAGACCATGGGGACTGGTCTTCCAAGCCTGAAGCGGGATCCATACATCTACCGTTCTGGCGT\
TCACTTGGCGCAGAGGTCTTTCCCGGTCTGACCCCGTGACCGAGAACTTTATGCCTGTCTGAATATACAGATTGACTGAC\
CATACTTCGATTACGGGCAGTCTTACCAAGATGAGATGCAATCACCATTTGGAACGAGAAGAGCATTGGGGACCCTAAAC\
A");
        let result = String::from("\
TGTTTAGGGTCCCCAATGCTCTTCTCGTTCCAAATGGTGATTGCATCTCATCTTGGTAAGACTGCCCGTAATCGAAGTAT\
GGTCAGTCAATCTGTATATTCAGACAGGCATAAAGTTCTCGGTCACGGGGTCAGACCGGGAAAGACCTCTGCGCCAAGTG\
AACGCCAGAACGGTAGATGTATGGATCCCGCTTCAGGCTTGGAAGACCAGTCCCCATGGTCTACCTTGAGCGTAGTCGGA\
TTCTTTAAAAACATTGAACCTGCATATCAGCGCGAGGCTGTGTCTCACATCGGCACCGCCCCATAGGCCTGCCCAGGATA\
GCCCTGAGCATTTAATTGGATGTATTATCGTTGTCTCCGCAGAGGCCTAGGATTGTCCCGACAGTTCCAGTGGTATATAG\
CGCCAATTCACCATCTTCCTCGTAAGCTTATAATATCTTTAGTTATCTTCAAGGGGGACATCACTTCACTAGAATTTCTG\
CCAAATGTTCACCTCTGGCTGTAGAGCTGCGCCTGGCCAGAAGATATTTAAATTAAAATAGGGACCCCATCACACGGAAC\
GGACACTGTAGAGTGTATATGTATCGAACATATTTAATGTTTGAATATCATCCTCTAAAAGACTAAGCAAAGTTTGTAGG\
GTCTCGCGCCAAAGCGCTAGATGATGGGGTATTGGTGAACATCCGACTGATTAATCTACGGAGCTAGCGTAATGGCATCT\
ATGGTGGACGACTGTGCTCCATGTCAGCGTGCATCTACCCCCAGTCCTTGCGACCTTCGTGGAGGCAACTATTATAGCGA\
TTTGACCAACTGGCGATTTGGAGTTAGACGCCAAGCCATTCCCTAAGACCCCCTCGGATAAAGACACTGCGCGAGGCTCA\
A");
        let mut problem = Problem::new();
        problem.get_solution(input);
        assert_eq!(problem, Problem { val: result });
    }
}
