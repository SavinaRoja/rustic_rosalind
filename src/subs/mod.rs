use super::Solvable;
use std::fmt::{Display, Formatter, Error};
use std::fs;
use bio::pattern_matching::shift_and;

pub type Problem = SolvableResult;

#[derive(Debug, PartialEq)]
pub struct SolvableResult{
    matches: Vec<usize>
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { matches: Vec::new() }
    }
}

impl Solvable for Problem {
    type Parameters = (String, String);

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let contents = fs::read_to_string(input_filename).unwrap();
        let mut tokens = contents.split_whitespace();
        let s = tokens.next().unwrap().to_string();
        let t = tokens.next().unwrap().to_string();
        return (s, t)
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let (s, t) = params;
        let shiftand = shift_and::ShiftAnd::new(t.as_bytes());
        for m in shiftand.find_all(s.as_bytes()) {
            self.matches.push(m + 1);
        }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let vec_length = &self.matches.len();

        if *vec_length == 0 as usize {
            write!(f, "")
        } else if *vec_length == 1 as usize {
            write!(f, "{}", &self.matches[0])
        } else {
            let mut joined = String::new();
            for val in &self.matches[..vec_length - 1] {
                joined.push_str(&(val.to_string())[..]);
                joined.push_str(" ");
            }
            let last_element = &self.matches.last().unwrap();
            joined.push_str(&(last_element.to_string())[..]);
            write!(f, "{}", joined)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_problem() {
        let problem = Problem::new();
        assert_eq!(problem, Problem { matches: Vec::new() });
    }

}
