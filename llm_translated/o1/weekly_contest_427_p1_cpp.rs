// Problem: Weekly Contest 427 Problem 1
// This Rust program replicates the exact input/output behavior of the given C++ code.
use std::io::{self, BufRead};

// A simple helper struct to handle token-based input reading.
// This replicates the behavior of C++'s >> operator, allowing
// us to read integers one by one, regardless of whether they
// appear on the same line or multiple lines in the input.
struct Scanner {
    tokens: Vec<String>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner { tokens: Vec::new() }
    }

    // Fetch the next token (replenish tokens if necessary).
    fn next_token(&mut self) -> String {
        while self.tokens.is_empty() {
            let mut input = String::new();
            // Read a line from stdin.
            io::stdin()
                .lock()
                .read_line(&mut input)
                .expect("Failed to read from stdin");
            // Split the line into whitespace-delimited tokens
            // and collect them into our tokens vector.
            self.tokens.extend(input.split_whitespace().map(String::from));
        }
        self.tokens.remove(0)
    }

    // Read the next i32 from stdin, using next_token().
    fn next_i32(&mut self) -> i32 {
        self.next_token().parse().expect("Failed to parse i32")
    }

    // Read the next usize from stdin (useful for lengths, etc.).
    fn next_usize(&mut self) -> usize {
        self.next_token().parse().expect("Failed to parse usize")
    }
}

// Solution struct replicating the class with the function in C++.
struct Solution;

impl Solution {
    // Converts the given vector A into the transformed array:
    // res[i] = A[(i + A[i] % n + n) % n].
    // This ensures we remain within bounds by using modular arithmetic.
    fn construct_transformed_array(&self, A: &[i32]) -> Vec<i32> {
        let n = A.len() as i32;
        let mut res = vec![0; n as usize];
        for i in 0..n as usize {
            let idx = (i as i32 + (A[i] % n) + n) % n;
            res[i] = A[idx as usize];
        }
        res
    }
}

fn main() {
    let mut scanner = Scanner::new();
    // Read the size of the array
    let n = scanner.next_usize();

    // Read the elements of the array
    let mut A = Vec::with_capacity(n);
    for _ in 0..n {
        A.push(scanner.next_i32());
    }

    // Construct the transformed array
    let solution = Solution;
    let transformed_array = solution.construct_transformed_array(&A);

    // Output the transformed array
    // (printing each number followed by a space, then a newline)
    for num in transformed_array {
        print!("{} ", num);
    }
    println!();
}