use std::fmt;
use std::fs;
use super::Solvable;

/// An alias for DNACounts, which implements Solvable
pub type Problem = DNACounts;

#[derive(Debug, PartialEq)]
pub struct DNACounts {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
}

/// Create a DNACounts baseline, 0 for all struct values
impl DNACounts{
    pub fn new() -> DNACounts {
        return DNACounts{ a: 0, c: 0, g: 0, t: 0 }
    }
}

impl fmt::Display for DNACounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.c, self.g, self.t)
    }
}

impl Solvable for DNACounts {
    type Parameters = String;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let contents = fs::read_to_string(input_filename).unwrap();
        return contents
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        for char in params.chars() {
            match char {
                'A' => {self.a += 1},
                'C' => {self.c += 1},
                'G' => {self.g += 1},
                'T' => {self.t += 1},
                _ => {},  // To make match exhaustive
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_empty() {
        let mut problem = Problem::new();
        let params = String::new();
        problem.get_solution(params);
        assert_eq!(problem, Problem::new());
    }

    #[test]
    fn dna_one_of_each() {
        let mut problem = Problem::new();
        let params = String::from("ACGT");
        problem.get_solution(params);
        assert_eq!(problem, DNACounts{ a: 1, c: 1, g: 1, t: 1});
    }

    #[test]
    fn dna_one_of_each_with_noise() {
        let mut problem = Problem::new();
        let params = String::from("1A.C-GaTh]");
        problem.get_solution(params);
        assert_eq!(problem, DNACounts{ a: 1, c: 1, g: 1, t: 1});
    }

    #[test]
    fn dna_none_with_noise() {
        let mut problem = Problem::new();
        let params = String::from("123><?[]eleminnow");
        problem.get_solution(params);
        assert_eq!(problem, Problem::new());
    }

    #[test]
    fn dna_with_a_full_dataset() {
        let mut problem = Problem::new();
        let params = String::from("\
ATGGCCAAATCCCTCTTACAATGAATAGTTGTGAATTATACACATGTGACTTTAGTAAGCCTTAAACTGATGCGTAAAG\
ATGAGCACAGCAGAGTGATGCGTTATGGATTGTCATAGTCCCAAGGGCTCTACGGATAATTGGTCAGGGATGGTTATCT\
TCTGGTGACGGGCGCCGATTCTTGAATCAGAGCCGTCGCACCCATTAACGCTTGAAGGTCGATTCCTTTCGAGTATTGC\
CTCACCGGATTAGCACTAGAGACCTAATGCGCCCGACAAGGGTATAAGTAGTCTGGGCTCTTAGCGTTTGGTCAAACTA\
CCCTTTATCGCCGACTAAGTTCCAGGTTGAGCCAACTCCGTCCGGAACGGTATCCGCTTGGCAACCCACCAGCTCGAAG\
CGGCGGTAGTCTATGAATTTAAATAGGACCGATTGGTCCTTCAGGCCTCGTTCTTAACCTTTGATAACCCGTTATCCAG\
GGTCCATGCTTTTTTTATCCCTAGCTGTCGCCGGGCGCAGGGACAGGTACTTCATGGCGCTAGTGATGTTAGTGCTGCT\
CCCCAAAGCTATCGGTAATATCCAGCATAAAAGGGACCCTTTGGATAGAGCAGTATGCAGAGTTCACTCGCGAGCGCTA\
CTAGTACAAGAACGAAGTTCACGTTCAGGCGTACGATTCGTAAATACCCAACGTCGTACTCCCGGACTCTGGAAGACGT\
CAAGTGATGCTCCAACTGTTTAAAAATATGTGGGGTTACCGATCGCGCGTTATAGGTCGGGATCCCGCTGGTGGGGGAA\
CATGCATAGCTCAAGGCGTTCGTCAGCTCGGTCTCCAGACCCGTTTAGCGACGCAAAGGGAATTGGGTACCCGCAGTCA\
ATTATCGCTGTCGATGGGGGTCTCGCTCTGATCCGTATTATCTCTAACGTCCGTTTGTCCAAAAGGTAAAATGTTGTGT\
AGCTGTTCGTTGT");
        problem.get_solution(params);
        assert_eq!(problem, DNACounts{ a:229, c: 230, g:246, t:256})
    }
}