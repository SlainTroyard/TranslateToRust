use std::io::{self, BufRead, Write};

/// Represents the solution for the problem.
struct Solution;

impl Solution {
    /// Determines the k-th character after a series of operations.
    fn kth_character(k: usize, operations: &Vec<i32>) -> char {
        let mut k = k - 1;
        let mut inc = 0;
        // Calculate the number of bits in k
        let mut i = 0;
        while (1 << i) <= k {
            i += 1;
        }
        for j in (0..i).rev() {
            if (k >> j) & 1 == 1 {
                inc += operations[j] as i32;
            }
        }
        // Convert the result to a character
        (b'a' + (inc % 26) as u8) as char
    }
}

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();
    stdin_lock.read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let operation_size: usize = iter.next().unwrap().parse().unwrap();

    let mut operations: Vec<i32> = vec![0; operation_size];
    for i in 0..operation_size {
        line.clear();
        stdin_lock.read_line(&mut line).unwrap();
        operations[i] = line.trim().parse().unwrap();
    }

    // Create an instance of Solution and call the kthCharacter method
    let s = Solution;
    let result = s.kth_character(k, &operations);

    // Write the result to stdout
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    writeln!(stdout_lock, "{}", result).unwrap();
}