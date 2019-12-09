use std::fmt;
use std::fs;

pub fn file_parse(input_filename: &str) -> String {
    let contents = fs::read_to_string(input_filename).unwrap();
    return contents
}

#[derive(Debug, PartialEq)]
pub struct DNACounts {
    a: u32,
    c: u32,
    g: u32,
    t: u32,
}

impl fmt::Display for DNACounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.a, self.c, self.g, self.t)
    }
}

pub fn dna(input: &String) -> DNACounts {

    let mut counts = DNACounts{ a: 0, c: 0, g: 0, t: 0 };

    for char in input.chars() {
        match char {
            'A' => {counts.a += 1},
            'C' => {counts.c += 1},
            'G' => {counts.g += 1},
            'T' => {counts.t += 1},
            _ => {},  // To make match exhaustive
        }
    }
    return counts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dna_empty() {
        let vals = dna(&String::new());
        assert_eq!(vals, DNACounts{ a: 0, c: 0, g: 0, t: 0});
    }

    #[test]
    fn dna_one_of_each() {
        let vals = dna(&String::from("ACGT"));
        assert_eq!(vals, DNACounts{ a: 1, c: 1, g: 1, t: 1});
    }

    #[test]
    fn dna_one_of_each_with_noise() {
        let vals = dna(&String::from("1A.C-GaTh]"));
        assert_eq!(vals, DNACounts{ a: 1, c: 1, g: 1, t: 1});
    }
    #[test]
    fn dna_none_with_noise() {
        let vals = dna(&String::from("123><?[]eleminnow"));
        assert_eq!(vals, DNACounts{ a: 0, c: 0, g: 0, t: 0});
    }

    #[test]
    fn dna_with_a_full_dataset() {
        let vals = dna(
            &String::from("\
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
AGCTGTTCGTTGT")
        );
        assert_eq!(vals, DNACounts{ a:229, c: 230, g:246, t:256})
    }
}