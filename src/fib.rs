use std::fs;

pub fn file_parse(input_filename: &str) -> (u32, u32) {
    let contents = fs::read_to_string(input_filename).unwrap();
    let n : u32;
    let k : u32;
    let mut tokens = contents.split_whitespace();
    n = tokens.next().unwrap().parse::<u32>().unwrap();
    k = tokens.next().unwrap().parse::<u32>().unwrap();
    return (n, k)
}

pub fn fib((n, k) : (u32, u32)) -> u32 {
    let mut a : u32 = 1;
    let mut b : u32 = 1;
    let mut c : u32;
    if n == 1 {
        return a
    } else if n == 2 {
        return b
    } else {
        for _ in 0..n-2 {
            c = b;
            b = a * k + b;
            a = c;
        }
        return b;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_n_one_k_one() {
        assert_eq!(fib((1, 1)), 1);
    }

    #[test]
    fn fib_n_two_k_one() {
        assert_eq!(fib((2, 1)), 1);
    }

    #[test]
    fn fib_n_one_k_ten() {
        assert_eq!(fib((1, 10)), 1);
    }

    #[test]
    fn fib_n_two_k_10() {
        assert_eq!(fib((2, 10)), 1);
    }

    #[test]
    fn fib_n_five_k_three() {
        assert_eq!(fib((5, 3)), 19);
    }

    #[test]
    fn fib_n_twenty_nine_k_two() {
        assert_eq!(fib((29, 2)), 178956971);
    }
}