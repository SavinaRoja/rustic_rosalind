use std::fs;
use std::fmt;
use super::Solvable;

pub type Problem = FibSolution;

#[derive(Debug, PartialEq)]
pub struct FibSolution {
    val: u32,
}

impl Problem {
    pub fn new() -> Problem {
        return Problem { val: 0 }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Solvable for Problem {
    type Parameters = (u32, u32);

    fn file_parse(&self, input_filename: &str) -> Self::Parameters {
        let contents = fs::read_to_string(input_filename).unwrap();
        let n : u32;
        let k : u32;
        let mut tokens = contents.split_whitespace();
        n = tokens.next().unwrap().parse::<u32>().unwrap();
        k = tokens.next().unwrap().parse::<u32>().unwrap();
        return (n, k)
    }

    fn get_solution(&mut self, params: Self::Parameters) {
        let (n, k) = params;
        let mut a : u32 = 1;
        let mut b : u32 = 1;
        let mut c : u32;
        if n == 1 {
            self.val = a;
            return ()
        } else if n == 2 {
            self.val = b;
            return ()
        } else {
            for _ in 0..n-2 {
                c = b;
                b = a * k + b;
                a = c;
            }
            self.val = b;
            return ()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_n_one_k_one() {
        let mut problem = Problem::new();
        problem.get_solution((1, 1));
        assert_eq!(problem, FibSolution { val: 1 });
    }

    #[test]
    fn fib_n_two_k_one() {
        let mut problem = Problem::new();
        problem.get_solution((2, 1));
        assert_eq!(problem, FibSolution { val: 1 });
    }

    #[test]
    fn fib_n_one_k_ten() {
        let mut problem = Problem::new();
        problem.get_solution((1, 10));
        assert_eq!(problem, FibSolution { val: 1 });
    }

    #[test]
    fn fib_n_two_k_10() {
        let mut problem = Problem::new();
        problem.get_solution((2, 10));
        assert_eq!(problem, FibSolution { val: 1 });
    }

    #[test]
    fn fib_n_five_k_three() {
        let mut problem = Problem::new();
        problem.get_solution((5, 3));
        assert_eq!(problem, FibSolution { val: 19 });
    }

    #[test]
    fn fib_n_twenty_nine_k_two() {
        let mut problem = Problem::new();
        problem.get_solution((29, 2));
        assert_eq!(problem, FibSolution { val: 178956971 });
    }
}