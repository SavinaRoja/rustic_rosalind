use super::Solvable;
use std::fmt::{Display, Formatter, Error};
use std::fs;

pub type Problem = ProbResult;

#[derive(Debug, PartialEq)]
pub struct ProbResult {
    s: String,
    probabilities: Vec<f32>,
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { s : String::new(), probabilities : Vec::new() }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let vec_length = &self.probabilities.len();

        if *vec_length == 0 as usize {
            write!(f, "")
        } else if *vec_length == 1 as usize {
            write!(f, "{}", &self.probabilities[0])
        } else {
            let mut joined = String::new();
            for val in &self.probabilities[..vec_length - 1] {
                joined.push_str(&(val.to_string())[..]);
                joined.push_str(" ");
            }
            let last_element = &self.probabilities.last().unwrap();
            joined.push_str(&(last_element.to_string())[..]);
            write!(f, "{}", joined)
        }
    }
}

impl Solvable for Problem {
    type Parameters = (String, Vec<f32>);

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let contents = fs::read_to_string(input_filename).unwrap();
        let mut tokens = contents.split_whitespace();
        let s = tokens.next().unwrap().to_string();
        let mut gc_contents: Vec<f32> = Vec::new();
        for t in tokens {
            gc_contents.push(t.parse::<f32>().unwrap());
        }
        return (s, gc_contents)
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let (s, gc_contents) = params;

        for gc in gc_contents {
            let gc_prob: f32 = (gc / 2.0).log10();
            let at_prob: f32 = (( 1.0 - gc ) / 2.0).log10();
            let mut prob: f32 = 0.0;
            for val in s.as_bytes() {
                match val {
                    65 => {
                        prob = prob + at_prob;
                    }
                    84 => {
                        prob = prob + at_prob;
                    }
                    67 => {
                        prob = prob + gc_prob;
                    }
                    71 => {
                        prob = prob + gc_prob;
                    }
                    _ => {}
                }
            }
            self.probabilities.push(prob);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve() {
        let mut problem = Problem::new();
        problem.get_solution((String::from("ACGT"), vec![0.5, 0.75]));
    }
}