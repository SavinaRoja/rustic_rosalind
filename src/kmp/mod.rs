use std::fmt::{Display, Formatter, Error};
use super::Solvable;
use std::iter::repeat;
use std::path::Path;
use bio::io::fasta;


pub type Problem = FailureArray;

#[derive(Debug, PartialEq)]
pub struct FailureArray {
    array: Vec<usize>
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let vec_length = &self.array.len();

        if *vec_length == 0 as usize {
            write!(f, "")
        } else if *vec_length == 1 as usize {
            write!(f, "{}", &self.array[0])
        } else {
            let mut joined = String::new();
            for val in &self.array[..vec_length - 1] {
                joined.push_str(&(val.to_string())[..]);
                joined.push_str(" ");
            }
            let last_element = &self.array.last().unwrap();
            joined.push_str(&(last_element.to_string())[..]);
            write!(f, "{}", joined)
        }
    }
}

impl Solvable for Problem {
    type Parameters = fasta::Record;

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let path_string = String::from(input_filename);
        let path = Path::new(&path_string);
        let reader = fasta::Reader::from_file(path).unwrap();

        return reader.records().next().unwrap().unwrap()
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let pattern = params.seq();
        let pattern_len = pattern.len();
        self.array = repeat(0).take(pattern_len).collect();
        let mut m: usize = 0;
        // i 0123456789
        // p CAGCATGGTATCACAGCAGAG
        // m 000120000001212345300
        for i in 1..pattern_len {
            while m > 0 && pattern[i] != pattern[m] {
                m = self.array[m - 1];
            }
            if pattern[i] == pattern[m] {
                m = m + 1;
            }
            self.array[i] = m;
        }
    }
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { array: Vec::new() }
    }
}