use super::Solvable;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::{Display, Formatter, Error};

pub type Problem = HammResult;

#[derive(Debug, PartialEq)]
pub struct HammResult {
    val: u32,
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { val: 0 }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.val)
    }
}

impl Solvable for Problem {
    type Parameters = (String, String);

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let file = File::open(input_filename).unwrap();
        let reader= BufReader::new(file);
        let mut lines = reader.lines().take(2);
        return (lines.next().unwrap().unwrap(),
                lines.next().unwrap().unwrap())
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let (line1, line2) = params;
        assert_eq!(line1.len(), line2.len());
        let zipped = line1.chars().zip(line2.chars());
        for pair in zipped {
            let (a, b) = pair;
            if a != b {self.val += 1}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamm_empty() {
        let mut problem = Problem::new();
        let params = (String::new(), String::new());
        problem.get_solution(params);
        assert_eq!(problem, Problem { val: 0});
    }

    #[test]
    fn hamm_simple_zero() {
        let mut problem = Problem::new();
        let params = (String::from("a"), String::from("a"));
        problem.get_solution(params);
        assert_eq!(problem, Problem { val: 0});
    }

    #[test]
    fn hamm_simple_one() {
        let mut problem = Problem::new();
        let params = (String::from("a"), String::from("b"));
        problem.get_solution(params);
        assert_eq!(problem, Problem { val: 1});
    }

    #[test]
    #[should_panic]
    fn hamm_length_mismatch() {
        let mut problem = Problem::new();
        let params = (String::from("a"), String::from(""));
        problem.get_solution(params);
    }

    #[test]
    fn hamm_full_dataset() {
        let line1 = String::from("\
TATTGCGTTTATGCGACGCAGCGGTGGTTTTGTCACGAATAAATGATATGCCGCTTCCTGGATCCCTCTCCACGTGGAGG\
GCCGATGCTGAGAATCAGGCCGCTTGGACTTGCGAACTTACCCTGGACCAAATATTCGTAGTCCTGTATTGCCCAGCAGG\
CTATGTGCCACAGAGGCCTCGGTAAATCGTGCTACATCACTCCGGGCACAACAACTGGCAGAGATACAATCGACATATGC\
GTACATGGGCTCCAGGACGGGTGATAGCTGTGAACCCGGAGTCCTTAATTTCTTTTCTATATCGAGACAAGACATGGAGA\
GCTTCTCTCCGCTACTGCCTCCGTTCACGCTGTACCAGCATAGTAAATCGTTGGGAGTTTCGAGCCTGCGGATCGGTCAT\
GCTGGCTGCAATCGTCAAGTTTCGAAGGTCCATACATGTATAACCGTAGACCTGTGGAGTGTCATGGGCCGCACCAGATT\
TCCAGGAGACACGCCTAGTACTTTCAGGGCCGCACTAATTCCTCCTCCCGCCGGAACAGCGAACTTGTCGGAGAGAGAAA\
TCTGTGCCCGTCCGCGCAGTCCACCGAGGCTATGATTAGCTTTTTGGCTTTAATCCGCCTCTTCTGGTTTCGAGGCGTAG\
CTGAATAGTTTACTTCGTAACGGTTCAGGCTCGTTAGGAAGCCCCGACGCTTACAGGCGGTTATTTGTCATATCTCTGAC\
GTGCGGGAGCCCGGCAACGGCACGCGTTAACTCCAATGCCTAAACTCGACAATTAAAAGATGTTAGGCTCGCGGTGGATA\
CGCAGCTTACTCCCTCTGCACGCCAACCTAGTGACAGTGCGGGAGTACACTTTATGGAAGATGACCGAGGAATTTCGACG\
GACCTCTTAGAATAGAACGCG");
        let line2 = String::from("\
GTTATCTGTCTCGTGCCTAAGGGGTTGCATCGTCTTGCAAATGCTGTATGCTGCCTACTCAGTCTGCCTATAAGTGGGTG\
CCCAATGTTTCGGATCAGTCCGCGTCCGCTTGCGGTGAAACTCTAGGCCCAATACTGGACGGCCTGCCCGGCCCAGCTGT\
CTGAGCGCCACCAAGTCTTCCGTCTCCAATCGCTCATACGTCCGAGAGCAACCACTTGAGATGATCGCATCGTGACCGGG\
TTCCCTGGCAGACTGCAAAGGGGTCTGATGCTCACATGATACCTTGAAAATCTGTTCTATATCACCAGAAGTAAAGTAAG\
GTCTCTCTCCACGTTACCTTACGTGCGAGCAATAGCATCATAAGCAAAGGATCTATAGCGGGAGGCTGCCTGCCCCGTTC\
ACTGGCGTTAATTGTCAACTGCTGGTGCCCGTTACTTGAGAACCCAAAGGCCAGTGCAGGCCCCTGCCTCAATCCAGTCT\
ACGTGAAGTATCGCCTCGCCCTTTAAAGAGCGGCATCGGTAGTGCCTCAGTCGTAACGACAGGGTGCGCGGCGACTGTTA\
CATCTACGATTACACCGAGTCCATGCAGGCGGTGACGATCGATTGGCAGATTAACCGACTATTATGCATTTGCGGCTAAG\
ACGCACAGGTTACTTCTTGACCCTACAAGCTGATGAGGCGCTACGGTTGGTCCCAGGCGACTACCTGTACTATGTCCGGC\
TCGCCGCGACACGGCAATTTATGCCTCTAGTCCCTACACCGAAAGTTGATAATTAGAACATCGTCGTCTCATAATGGCTG\
GCATGTATACGCTCACTGCGCGGCCGTCCAGGACATCCACGGGAATAATTAAGATCGTTGATCACGTAGCAATTGAGAGG\
GACCGAATAGATTAGGTGGCA");
        let mut problem = Problem::new();
        problem.get_solution((line1, line2));
        assert_eq!(problem, Problem { val: 430})
    }
}