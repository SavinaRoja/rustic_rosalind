use super::Solvable;
use std::fmt::{Display, Formatter, Error};
use std::collections::HashMap;

pub type Problem = ProtResult;

#[derive(Debug, PartialEq)]
pub struct ProtResult {
    protein_seq: String,
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.protein_seq)
    }
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { protein_seq: String::new() }
    }
}

    fn translate(rna_seq: String) -> String {
        let codon_table: HashMap<&str, &str> = [
            ("UUU", "F"), ("CUU", "L"), ("AUU", "I"), ("GUU", "V"),
            ("UUC", "F"), ("CUC", "L"), ("AUC", "I"), ("GUC", "V"),
            ("UUA", "L"), ("CUA", "L"), ("AUA", "I"), ("GUA", "V"),
            ("UUG", "L"), ("CUG", "L"), ("AUG", "M"), ("GUG", "V"),
            ("UCU", "S"), ("CCU", "P"), ("ACU", "T"), ("GCU", "A"),
            ("UCC", "S"), ("CCC", "P"), ("ACC", "T"), ("GCC", "A"),
            ("UCA", "S"), ("CCA", "P"), ("ACA", "T"), ("GCA", "A"),
            ("UCG", "S"), ("CCG", "P"), ("ACG", "T"), ("GCG", "A"),
            ("UAU", "Y"), ("CAU", "H"), ("AAU", "N"), ("GAU", "D"),
            ("UAC", "Y"), ("CAC", "H"), ("AAC", "N"), ("GAC", "D"),
            ("UAA", "Stop"), ("CAA", "Q"), ("AAA", "K"), ("GAA", "E"),
            ("UAG", "Stop"), ("CAG", "Q"), ("AAG", "K"), ("GAG", "E"),
            ("UGU", "C"), ("CGU", "R"), ("AGU", "S"), ("GGU", "G"),
            ("UGC", "C"), ("CGC", "R"), ("AGC", "S"), ("GGC", "G"),
            ("UGA", "Stop"), ("CGA", "R"), ("AGA", "R"), ("GGA", "G"),
            ("UGG", "W"), ("CGG", "R"), ("AGG", "R"), ("GGG", "G"),
        ].iter().cloned().collect();
        let mut protein_seq = String::new();
        let mut rna_chars = rna_seq.chars();

        loop {
            let mut codon = String::new();
            for _ in 0..3 {
                let nucleic_acid = rna_chars.next();
                match nucleic_acid {
                    Some(na) => { codon.push(na)},
                    None => return protein_seq
                }
            }
            let amino_acid = codon_table.get(codon.as_str());
            match amino_acid {
                Some(aa) => {
                    if *aa == "Stop" {
                        return protein_seq
                    } else {
                        protein_seq.push(aa.parse().unwrap())
                    }
                },
                None => {return protein_seq}
            }
        }
    }

impl Solvable for Problem {
    type Parameters = String;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        return std::fs::read_to_string(input_filename).unwrap()
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let rna = params;
        self.protein_seq = translate(rna);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prot_empty() {
        let mut problem = Problem::new();
        problem.get_solution(String::new());
        assert_eq!(problem, Problem { protein_seq: String::new() });
    }

    #[test]
    fn prot_one_na() {
        let mut problem = Problem::new();
        problem.get_solution(String::from("A"));
        assert_eq!(problem, Problem { protein_seq: String::new() });
    }

    #[test]
    fn prot_two_na() {
        let mut problem = Problem::new();
        problem.get_solution(String::from("AU"));
        assert_eq!(problem, Problem { protein_seq: String::new() });
    }

    #[test]
    fn prot_one_codon() {
        let mut problem = Problem::new();
        problem.get_solution(String::from("AUG"));
        assert_eq!(problem, Problem { protein_seq: String::from("M") });
    }

    #[test]
    fn prot_short_sequence() {
        let mut problem = Problem::new();
        problem.get_solution(String::from("AUGUAUGACAGGACGUCAUUA"));
        assert_eq!(problem, Problem { protein_seq: String::from("MYDRTSL") })
    }

    #[test]
    fn prot_early_termination() {
        let mut problem = Problem::new();
        problem.get_solution(String::from("AUGUAUGACAGGUGAUCAUUA"));
        assert_eq!(problem, Problem { protein_seq: String::from("MYDR") })
    }
}