use super::Solvable;
use std::fmt::{Display, Formatter, Error};

pub type Problem = IprbResult;

#[derive(Debug, PartialEq)]
pub struct IprbResult {
    probability: f32,
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { probability: 0.0}
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.probability)
    }
}

impl Solvable for Problem {
    type Parameters = (f32, f32, f32);

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let contents = std::fs::read_to_string(input_filename).unwrap();
        let mut tokens = contents.split_whitespace();
        let k: f32 = tokens.next().unwrap().parse().unwrap();
        let m: f32 = tokens.next().unwrap().parse().unwrap();
        let n: f32 = tokens.next().unwrap().parse().unwrap();
        return (k, m, n)
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let (k, m, n) = params;
        let pop = k + m + n;
        self.probability = 1.0 - (
            ((m * n) + (0.25 * m * (m - 1.0)) + (n * (n - 1.0))) /
                (pop * (pop - 1.0))
        );
    }
}
